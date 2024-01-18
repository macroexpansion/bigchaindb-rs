use std::collections::HashSet;

use sha2::{Digest, Sha256};

use crate::{condition::Condition, schema::fingerprint::Fingerprint};

pub trait Fulfillment: Fingerprint {
    const TYPE_ID: usize;
    const TYPE_NAME: &'static str;
    const CONSTANT_COST: usize;

    fn get_type_id(&self) -> usize {
        Self::TYPE_ID
    }

    fn get_type_name(&self) -> &'static str {
        Self::TYPE_NAME
    }

    fn get_condition(&self) -> Condition {
        Condition {
            hash: self.generate_hash(),
            type_id: self.get_type_id(),
            cost: self.caculate_cost(),
            subtypes: HashSet::new(),
        }
    }

    fn get_condition_uri(&self) -> String {
        self.get_condition().serialize_uri()
    }

    fn generate_hash(&self) -> [u8; 32] {
        let mut hash = Sha256::new();
        hash.update(self.get_fingerprint_contents());

        let mut buffer = [0u8; 32];
        let digest = hash.finalize();
        buffer.copy_from_slice(&digest[..]);

        buffer
    }

    fn caculate_cost(&self) -> usize {
        Self::CONSTANT_COST
    }
}
