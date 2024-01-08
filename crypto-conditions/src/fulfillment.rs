use std::collections::HashSet;

use crate::condition::Condition;

pub trait Fulfillment {
    const TYPE_ID: usize;
    const TYPE_NAME: &'static str;

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

    fn generate_hash(&self) -> [u8; 32];
    fn caculate_cost(&self) -> usize;
}
