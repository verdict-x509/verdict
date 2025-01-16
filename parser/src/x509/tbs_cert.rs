use vstd::prelude::*;

use crate::asn1::*;
use crate::asn1::Integer;

use crate::common::*;
use super::*;

verus! {

asn1! {
    seq TBSCertificate {
        #[default(0i64)] version: ASN1<ExplicitTag<ASN1<Integer>>> = ASN1(ExplicitTag(tag_of!(EXPLICIT 0), ASN1(Integer))),

        serial: ASN1<BigInt> = ASN1(BigInt),
        signature: Cached<ASN1<AlgorithmIdentifier>> = Cached(ASN1(AlgorithmIdentifier)),
        issuer: ASN1<Name> = ASN1(Name),
        validity: ASN1<Validity> = ASN1(Validity),
        subject: ASN1<Name> = ASN1(Name),
        subject_key: ASN1<PublicKeyInfo> = ASN1(PublicKeyInfo),

        #[optional] issuer_uid: ASN1<ImplicitTag<BitString>> = ASN1(ImplicitTag(tag_of!(IMPLICIT 1), BitString)),
        #[optional] subject_uid: ASN1<ImplicitTag<BitString>> = ASN1(ImplicitTag(tag_of!(IMPLICIT 2), BitString)),
        #[optional] extensions: ASN1<ExplicitTag<ASN1<Extensions>>> = ASN1(ExplicitTag(tag_of!(EXPLICIT 3), ASN1(Extensions))),
    }
}

}

#[cfg(test)]
mod test {
    use super::*;

    verus! {
        /// Check that all trait bounds and preconditions are satisfied
        #[test]
        fn is_combinator() {
            let _ = ASN1(TBSCertificate).parse(&[]);
        }
    }

    #[test]
    fn sanity() {
        assert!(ASN1(TBSCertificate).parse(&[
            0x30, 0x82, 0x04, 0x17, 0xA0, 0x03, 0x02, 0x01, 0x02, 0x02, 0x08, 0x25, 0xA1, 0xDF, 0xCA, 0x33, 0xCB, 0x59, 0x02, 0x30, 0x0D, 0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0B, 0x05, 0x00, 0x30, 0x81, 0xA4, 0x31, 0x0B, 0x30, 0x09, 0x06, 0x03, 0x55, 0x04, 0x06, 0x13, 0x02, 0x50, 0x41, 0x31, 0x0F, 0x30, 0x0D, 0x06, 0x03, 0x55, 0x04, 0x08, 0x0C, 0x06, 0x50, 0x61, 0x6E, 0x61, 0x6D, 0x61, 0x31, 0x14, 0x30, 0x12, 0x06, 0x03, 0x55, 0x04, 0x07, 0x0C, 0x0B, 0x50, 0x61, 0x6E, 0x61, 0x6D, 0x61, 0x20, 0x43, 0x69, 0x74, 0x79, 0x31, 0x24, 0x30, 0x22, 0x06, 0x03, 0x55, 0x04, 0x0A, 0x0C, 0x1B, 0x54, 0x72, 0x75, 0x73, 0x74, 0x43, 0x6F, 0x72, 0x20, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6D, 0x73, 0x20, 0x53, 0x2E, 0x20, 0x64, 0x65, 0x20, 0x52, 0x2E, 0x4C, 0x2E, 0x31, 0x27, 0x30, 0x25, 0x06, 0x03, 0x55, 0x04, 0x0B, 0x0C, 0x1E, 0x54, 0x72, 0x75, 0x73, 0x74, 0x43, 0x6F, 0x72, 0x20, 0x43, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x41, 0x75, 0x74, 0x68, 0x6F, 0x72, 0x69, 0x74, 0x79, 0x31, 0x1F, 0x30, 0x1D, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0C, 0x16, 0x54, 0x72, 0x75, 0x73, 0x74, 0x43, 0x6F, 0x72, 0x20, 0x52, 0x6F, 0x6F, 0x74, 0x43, 0x65, 0x72, 0x74, 0x20, 0x43, 0x41, 0x2D, 0x32, 0x30, 0x1E, 0x17, 0x0D, 0x31, 0x36, 0x30, 0x32, 0x30, 0x34, 0x31, 0x32, 0x33, 0x32, 0x32, 0x33, 0x5A, 0x17, 0x0D, 0x33, 0x34, 0x31, 0x32, 0x33, 0x31, 0x31, 0x37, 0x32, 0x36, 0x33, 0x39, 0x5A, 0x30, 0x81, 0xA4, 0x31, 0x0B, 0x30, 0x09, 0x06, 0x03, 0x55, 0x04, 0x06, 0x13, 0x02, 0x50, 0x41, 0x31, 0x0F, 0x30, 0x0D, 0x06, 0x03, 0x55, 0x04, 0x08, 0x0C, 0x06, 0x50, 0x61, 0x6E, 0x61, 0x6D, 0x61, 0x31, 0x14, 0x30, 0x12, 0x06, 0x03, 0x55, 0x04, 0x07, 0x0C, 0x0B, 0x50, 0x61, 0x6E, 0x61, 0x6D, 0x61, 0x20, 0x43, 0x69, 0x74, 0x79, 0x31, 0x24, 0x30, 0x22, 0x06, 0x03, 0x55, 0x04, 0x0A, 0x0C, 0x1B, 0x54, 0x72, 0x75, 0x73, 0x74, 0x43, 0x6F, 0x72, 0x20, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6D, 0x73, 0x20, 0x53, 0x2E, 0x20, 0x64, 0x65, 0x20, 0x52, 0x2E, 0x4C, 0x2E, 0x31, 0x27, 0x30, 0x25, 0x06, 0x03, 0x55, 0x04, 0x0B, 0x0C, 0x1E, 0x54, 0x72, 0x75, 0x73, 0x74, 0x43, 0x6F, 0x72, 0x20, 0x43, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x41, 0x75, 0x74, 0x68, 0x6F, 0x72, 0x69, 0x74, 0x79, 0x31, 0x1F, 0x30, 0x1D, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0C, 0x16, 0x54, 0x72, 0x75, 0x73, 0x74, 0x43, 0x6F, 0x72, 0x20, 0x52, 0x6F, 0x6F, 0x74, 0x43, 0x65, 0x72, 0x74, 0x20, 0x43, 0x41, 0x2D, 0x32, 0x30, 0x82, 0x02, 0x22, 0x30, 0x0D, 0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01, 0x05, 0x00, 0x03, 0x82, 0x02, 0x0F, 0x00, 0x30, 0x82, 0x02, 0x0A, 0x02, 0x82, 0x02, 0x01, 0x00, 0xA7, 0x20, 0x6E, 0xC2, 0x2A, 0xA2, 0x62, 0x24, 0x95, 0x90, 0x76, 0xC8, 0x38, 0x7E, 0x80, 0xD2, 0xAB, 0xC1, 0x9B, 0x65, 0x05, 0x94, 0xF4, 0xC1, 0x0A, 0x10, 0xD5, 0x02, 0xAC, 0xED, 0x9F, 0x93, 0xC7, 0x87, 0xC8, 0xB0, 0x27, 0x2B, 0x42, 0x0C, 0x3D, 0x0A, 0x3E, 0x41, 0x5A, 0x9E, 0x75, 0xDD, 0x8D, 0xCA, 0xE0, 0x9B, 0xEC, 0x68, 0x32, 0xA4, 0x69, 0x92, 0x68, 0x8C, 0x0B, 0x81, 0x0E, 0x56, 0xA0, 0x3E, 0x1A, 0xDD, 0x2C, 0x25, 0x14, 0x82, 0x2F, 0x97, 0xD3, 0x64, 0x46, 0xF4, 0x54, 0xA9, 0xDC, 0x3A, 0x54, 0x2D, 0x31, 0x2B, 0x99, 0x82, 0xF2, 0xD9, 0x2A, 0xD7, 0xEF, 0x71, 0x00, 0xB8, 0x31, 0xA4, 0xBE, 0x7A, 0x24, 0x07, 0xC3, 0x42, 0x20, 0xF2, 0x8A, 0xD4, 0x92, 0x04, 0x1B, 0x65, 0x56, 0x4C, 0x6C, 0xD4, 0xFB, 0xB6, 0x61, 0x5A, 0x47, 0x23, 0xB4, 0xD8, 0x69, 0xB4, 0xB7, 0x3A, 0xD0, 0x74, 0x3C, 0x0C, 0x75, 0xA1, 0x8C, 0x4E, 0x76, 0xA1, 0xE9, 0xDB, 0x2A, 0xA5, 0x3B, 0xFA, 0xCE, 0xB0, 0xFF, 0x7E, 0x6A, 0x28, 0xFD, 0x27, 0x1C, 0xC8, 0xB1, 0xE9, 0x29, 0xF1, 0x57, 0x6E, 0x64, 0xB4, 0xD0, 0xC1, 0x15, 0x6D, 0x0E, 0xBE, 0x2E, 0x0E, 0x46, 0xC8, 0x5E, 0xF4, 0x51, 0xFE, 0xEF, 0x0E, 0x63, 0x3A, 0x3B, 0x71, 0xBA, 0xCF, 0x6F, 0x59, 0xCA, 0x0C, 0xE3, 0x9B, 0x5D, 0x49, 0xB8, 0x4C, 0xE2, 0x57, 0xB1, 0x98, 0x8A, 0x42, 0x57, 0x9C, 0x76, 0xEF, 0xEF, 0xBD, 0xD1, 0x68, 0xA8, 0xD2, 0xF4, 0x09, 0xBB, 0x77, 0x35, 0xBE, 0x25, 0x82, 0x08, 0xC4, 0x16, 0x2C, 0x44, 0x20, 0x56, 0xA9, 0x44, 0x11, 0x77, 0xEF, 0x5D, 0xB4, 0x1D, 0xAA, 0x5E, 0x6B, 0x3E, 0x8B, 0x32, 0xF6, 0x07, 0x2F, 0x57, 0x04, 0x92, 0xCA, 0xF5, 0xFE, 0x9D, 0xC2, 0xE9, 0xE8, 0xB3, 0x8E, 0x4C, 0x4B, 0x02, 0x31, 0xD9, 0xE4, 0x3C, 0x48, 0x82, 0x27, 0xF7, 0x18, 0x82, 0x76, 0x48, 0x3A, 0x71, 0xB1, 0x13, 0xA1, 0x39, 0xD5, 0x2E, 0xC5, 0x34, 0xC2, 0x1D, 0x62, 0x85, 0xDF, 0x03, 0xFE, 0x4D, 0xF4, 0xAF, 0x3D, 0xDF, 0x5C, 0x5B, 0x8D, 0xFA, 0x70, 0xE1, 0xA5, 0x7E, 0x27, 0xC7, 0x86, 0x2E, 0x6A, 0x8F, 0x12, 0xC6, 0x84, 0x5E, 0x43, 0x51, 0x50, 0x9C, 0x19, 0x9B, 0x78, 0xE6, 0xFC, 0xF6, 0xED, 0x47, 0x7E, 0x7B, 0x3D, 0x66, 0xEF, 0x13, 0x13, 0x88, 0x5F, 0x3C, 0xA1, 0x63, 0xFB, 0xF9, 0xAC, 0x87, 0x35, 0x9F, 0xF3, 0x82, 0x9E, 0xA4, 0x3F, 0x0A, 0x9C, 0x31, 0x69, 0x8B, 0x99, 0xA4, 0x88, 0x4A, 0x8E, 0x6E, 0x66, 0x4D, 0xEF, 0x16, 0xC4, 0x0F, 0x79, 0x28, 0x21, 0x60, 0x0D, 0x85, 0x16, 0x7D, 0xD7, 0x54, 0x38, 0xF1, 0x92, 0x56, 0xFD, 0xB5, 0x33, 0x4C, 0x83, 0xDC, 0xD7, 0x10, 0x9F, 0x4B, 0xFD, 0xC6, 0xF8, 0x42, 0xBD, 0xBA, 0x7C, 0x73, 0x02, 0xE0, 0xFF, 0x7D, 0xCD, 0x5B, 0xE1, 0xD4, 0xAC, 0x61, 0x7B, 0x57, 0xD5, 0x4A, 0x7B, 0x5B, 0xD4, 0x85, 0x58, 0x27, 0x5D, 0xBF, 0xF8, 0x2B, 0x60, 0xAC, 0xA0, 0x26, 0xAE, 0x14, 0x21, 0x27, 0xC6, 0x77, 0x9A, 0x33, 0x80, 0x3C, 0x5E, 0x46, 0x3F, 0xF7, 0xC3, 0xB1, 0xA3, 0x86, 0x33, 0xC6, 0xE8, 0x5E, 0x0D, 0xB9, 0x35, 0x2C, 0xAA, 0x46, 0xC1, 0x85, 0x02, 0x75, 0x80, 0xA0, 0xEB, 0x24, 0xFB, 0x15, 0xAA, 0xE4, 0x67, 0x7F, 0x6E, 0x77, 0x3F, 0xF4, 0x04, 0x8A, 0x2F, 0x7C, 0x7B, 0xE3, 0x17, 0x61, 0xF0, 0xDD, 0x09, 0xA9, 0x20, 0xC8, 0xBE, 0x09, 0xA4, 0xD0, 0x7E, 0x44, 0xC3, 0xB2, 0x30, 0x4A, 0x38, 0xAA, 0xA9, 0xEC, 0x18, 0x9A, 0x07, 0x82, 0x2B, 0xDB, 0xB8, 0x9C, 0x18, 0xAD, 0xDA, 0xE0, 0x46, 0x17, 0xAC, 0xCF, 0x5D, 0x02, 0x03, 0x01, 0x00, 0x01, 0xA3, 0x63, 0x30, 0x61, 0x30, 0x1D, 0x06, 0x03, 0x55, 0x1D, 0x0E, 0x04, 0x16, 0x04, 0x14, 0xD9, 0xFE, 0x21, 0x40, 0x6E, 0x94, 0x9E, 0xBC, 0x9B, 0x3D, 0x9C, 0x7D, 0x98, 0x20, 0x19, 0xE5, 0x8C, 0x30, 0x62, 0xB2, 0x30, 0x1F, 0x06, 0x03, 0x55, 0x1D, 0x23, 0x04, 0x18, 0x30, 0x16, 0x80, 0x14, 0xD9, 0xFE, 0x21, 0x40, 0x6E, 0x94, 0x9E, 0xBC, 0x9B, 0x3D, 0x9C, 0x7D, 0x98, 0x20, 0x19, 0xE5, 0x8C, 0x30, 0x62, 0xB2, 0x30, 0x0F, 0x06, 0x03, 0x55, 0x1D, 0x13, 0x01, 0x01, 0xFF, 0x04, 0x05, 0x30, 0x03, 0x01, 0x01, 0xFF, 0x30, 0x0E, 0x06, 0x03, 0x55, 0x1D, 0x0F, 0x01, 0x01, 0xFF, 0x04, 0x04, 0x03, 0x02, 0x01, 0x86,
        ]).is_ok());
    }
}
