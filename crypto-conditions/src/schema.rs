pub mod fingerprint {
    use asn1::*;

    pub trait Fingerprint {
        fn get_fingerprint_contents(&self) -> Vec<u8>;
    }

    #[derive(Asn1Write)]
    pub struct Ed25519FingerprintContents<'a> {
        #[implicit(0)]
        pub public_key: Option<&'a [u8]>,
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use asn1::*;

        #[test]
        fn test_ed25519_fingerprint_contents() {
            let bytes = [1u8; 32];

            let buffer = write_single(&Ed25519FingerprintContents {
                public_key: Some(&bytes[..]),
            })
            .unwrap();

            assert_eq!(
                buffer,
                [
                    48, 34, 128, 32, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
                ]
            );
        }
    }
}
