use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process::{ExitCode, Command, Stdio};
use std::time::Instant;

use regex::Regex;
use colored::*;

/// Parse next_arg as <extern_name>=<prefix>-<extern_hash>.<ext>
/// and return (extern_name, extern_hash)
fn parse_extern_argument(s: &str) -> Option<(&str, &str)> {
    let (name, rest) = s.split_once('=')?;
    let (rest, _) = rest.rsplit_once('.')?;
    let (_, hash) = rest.rsplit_once('-')?;
    Some((name, hash))
}

/// Check if a string is a rustc artifact message
fn is_artifact_message(s: &str) -> bool {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(s) {
        if let Some(msg_type) = value.get("$message_type") {
            return msg_type.as_str() == Some("artifact");
        }
    }
    false
}

/// Tries to get additional Verus flags from the crate's Cargo.toml
fn get_verus_flags(path: &str) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    let parsed = toml::from_str::<toml::Value>(&content).ok()?;

    if let Some(section) = parsed.get("verus") {
        if let Some(flags) = section.get("extra_flags") {
            return Some(flags.as_str()?.to_string());
        }
    }

    None
}

enum Level {
    Note,
    Error,
}

/// Fake a Cargo message
fn cargo_message(level: Level, banner: &str, msg: &str) {
    println!("{:>12} {}",
        match level {
            Level::Note => banner.bold().bright_green(),
            Level::Error => banner.bold().bright_red(),
        },
        msg,
    )
}


/// Based on the arguments to rustc, call Verus if the given crate should be verified
fn check_verification(args: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut verus_args = Vec::new();
    let mut use_verus = false;

    let mut deps_dir = None;
    let mut hash = None;

    let mut iter = args.iter().peekable();

    // Filter arguments for Verus
    while let Some(arg) = iter.next() {

        // Process some special cases
        if arg == "--extern" {
            if let Some(next_arg) = iter.peek() {
                if next_arg.starts_with("vstd=") {
                    // Call Verus iff --extern vstd=... is part of the arguments
                    use_verus = true;
                }

                // Remove --extern vstd/builtin/builtin_macros=...
                // when calling Verus (otherwise all the Verus code
                // would be stripped)
                if next_arg.starts_with("vstd=")
                    || next_arg.starts_with("builtin=")
                    || next_arg.starts_with("builtin_macros=")
                {
                    iter.next();
                    continue;
                }

                // If verusdata exists, we assume that the extern crate is a Verus project
                // so we need to import .verusdata
                if let Some((name, hash)) = parse_extern_argument(next_arg) {
                    if let Some(deps_dir) = &deps_dir {
                        let verusdata_path = format!("{}/verify/{}-{}.verusdata", deps_dir, name, hash);
                        let verus_rmeta_path = format!("{}/verify/lib{}-{}.rmeta", deps_dir, name, hash);

                        if Path::new(&verusdata_path).exists() && Path::new(&verus_rmeta_path).exists() {
                            verus_args.push("--import".to_string());
                            verus_args.push(format!("{}={}", name, verusdata_path));
                            verus_args.push("--extern".to_string());
                            verus_args.push(format!("{}={}", name, verus_rmeta_path));
                            iter.next();
                            continue;
                        }
                    }
                }
            }
        } else if arg == "-C" {
            // Try to find the hash of the current crate being built
            if let Some(next_arg) = iter.peek() {
                if next_arg.starts_with("metadata=") {
                    hash = Some(next_arg["metadata=".len()..].to_string());
                }
            }
        } else if arg.starts_with("--edition=") {
            // Ignore --edition=* arguments, since Verus already provides it to rustc
            continue;
        } else if arg == "--out-dir" {
            // Rewrite --out-dir <dir> to --output-dir <dir>/verify
            // and also record <dir>
            if let Some(next_arg) = iter.peek() {
                deps_dir = Some(next_arg.to_string());
                verus_args.push("--out-dir".to_string());
                verus_args.push(format!("{}/verify", next_arg));
                iter.next();
                continue;
            }
        } else if arg.starts_with("--emit=") {
            // Overwrite --emit flags
            continue;
        }

        // Otherwise just use the same argument
        verus_args.push(arg.clone());
    }

    // Prepare and call verus command
    if use_verus && deps_dir.is_some() && hash.is_some() {
        let deps_dir = deps_dir.unwrap();
        let hash = hash.unwrap();

        let crate_name = env::var("CARGO_CRATE_NAME")?;
        let crate_version = env::var("CARGO_PKG_VERSION")?;
        let crate_path = env::var("CARGO_MANIFEST_DIR")?;

        cargo_message(Level::Note, "Verifying", &format!("{} v{} ({})", crate_name, crate_version, crate_path));

        // Create deps_dir/verify if it does not exist
        let verify_deps_dir = format!("{}/verify", deps_dir);
        fs::create_dir_all(&verify_deps_dir)?;

        let verusdata_path = format!(
            "{}/{}-{}.verusdata",
            verify_deps_dir, crate_name, hash
        );

        let mut verus_cmd = Command::new("verus");
        verus_cmd
            .env_remove("CARGO_MAKEFLAGS")
            .args(&verus_args)
            .arg("-L").arg(format!("dependency={}", verify_deps_dir))
            .arg("--emit=dep-info,metadata") // Don't do any linking
            .arg("--no-report-long-running")
            .arg("--compile")
            .arg("--export").arg(&verusdata_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // Add optional flags from VERUS_FLAGS
        if let Ok(flags) = env::var("VERUS_FLAGS") {
            verus_cmd.args(shell_words::split(&flags)?);
        }

        // Add optional flags from Cargo.toml
        if let Some(flags) = get_verus_flags(&format!("{}/Cargo.toml", crate_path)) {
            verus_cmd.args(shell_words::split(&flags)?);
        }

        // println!("running verus: {:?}", verus_cmd);
        let start = Instant::now();
        let mut verus_proc = verus_cmd.spawn()?;

        let verus_stderr = verus_proc.stderr.take()
            .ok_or(io::Error::other("failed to get stderr of verus"))?;
        let reader = BufReader::new(verus_stderr);

        // Filter JSON messages in stderr (ignore artifact messages)
        for line in reader.lines() {
            if let Ok(line) = line {
                if !is_artifact_message(&line) {
                    eprintln!("{}", &line);
                }
            }
        }

        let verus_stdout = verus_proc.stdout.take()
            .ok_or(io::Error::other("failed to get stdout of verus"))?;
        let reader = BufReader::new(verus_stdout);
        let result_re = Regex::new(r"^verification results:: (\d+) verified, (\d+) errors$")?;

        for line in reader.lines() {
            if let Ok(line) = line {
                if let Some(cap) = result_re.captures(&line) {
                    if let (Some(num_suc), Some(num_fail)) = (cap.get(1), cap.get(2)) {
                        let elapsed = start.elapsed().as_secs_f64();

                        let num_suc: usize = num_suc.as_str().parse()?;
                        let num_fail: usize = num_fail.as_str().parse()?;

                        if num_fail == 0 {
                            cargo_message(Level::Note, "Verus", &format!("{}: {} verified in {:.2}s", crate_name, num_suc, elapsed));
                        } else {
                            cargo_message(Level::Error, "Verus", &format!("{}: {} verified, {} failed, in {:.2}s", crate_name, num_suc, num_fail, elapsed));
                        }
                    }
                }
            }
        }

        let verus_status = verus_proc.wait()?;
        if !verus_status.success() {
            Err(io::Error::other("failed with non-zero exit code"))?
        }
    }

    Ok(())
}

fn main() -> ExitCode {
    colored::control::set_override(true);

    let mut args = env::args().skip(1);

    let rustc_path = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("rustc path should be provided as the first argument; use this command for RUSTC_WRAPPER only");
            return ExitCode::from(1);
        }
    };

    let rustc_args: Vec<String> = args.collect();

    if let Err(err) = check_verification(&rustc_args) {
        cargo_message(Level::Error, "Verus", &format!("{}", err));
        return ExitCode::from(1);
    }

    // Always call rustc at the end
    return ExitCode::from(Command::new(&rustc_path)
        .args(&rustc_args)
        .status()
        .expect("failed to execute rustc")
        .code()
        .unwrap_or(1) as u8);
}
