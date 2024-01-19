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

pub mod fulfillment {
    use asn1::*;

    #[derive(Asn1Write)]
    pub struct Ed25519Sha256Fulfillment<'a> {
        #[implicit(0)]
        pub public_key: Option<&'a [u8]>,
        #[implicit(1)]
        pub signature: Option<&'a [u8]>,
    }

    #[derive(Asn1Write)]
    pub enum FulfillmentChoice<'a> {
        // TODO: implement other choice
        // preimageSha256Fulfillment: this.implicit(0).use(PreimageFulfillment),
        // prefixSha256Fulfillment: this.implicit(1).use(PrefixFulfillment),
        // thresholdSha256Fulfillment: this.implicit(2).use(ThresholdFulfillment),
        // rsaSha256Fulfillment: this.implicit(3).use(RsaSha256Fulfillment),
        #[implicit(4)]
        Ed25519Sha256Fulfillment(Ed25519Sha256Fulfillment<'a>),
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_ed25519sha256_fulfillment() {
            let bytes = [1u8; 32];

            let fulfillment = Ed25519Sha256Fulfillment {
                public_key: Some(&bytes[..]),
                signature: Some(&bytes[..]),
            };

            let buffer = write_single(&fulfillment).unwrap();

            assert_eq!(bs58::encode(buffer).into_string(), "382G1e9VCNbaKW7vC3hW2zzySF2uofrQNGV1fNvpq8xbysNitah3euMnpvrQLjmASbz5CQyF7iSqfdaMr1Ds3BvynZyuBRrx");
        }

        #[test]
        fn test_fulfillment() {
            let bytes = [1u8; 32];
            let fulfillment = Ed25519Sha256Fulfillment {
                public_key: Some(&bytes[..]),
                signature: Some(&bytes[..]),
            };
            let choice = FulfillmentChoice::Ed25519Sha256Fulfillment(fulfillment);
            let buffer = write_single(&choice).unwrap();

            assert_eq!(bs58::encode(buffer).into_string(), "8DgD5ZffDSjWD1EyfqNwv5WkRJduNNo9YQvpLpu9akNzYM5tryitvi7yv9DSAh2kXouC8FDqWJkFtUXzkLoxiJoSofTJBsex")
        }
    }
}
