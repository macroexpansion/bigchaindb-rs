use std::collections::HashSet;

use crate::util::Base64Url;

pub struct Condition {
    pub hash: [u8; 32],
    pub type_id: usize,
    pub cost: usize,
    pub subtypes: HashSet<String>,
}

impl Condition {
    pub fn set_hash(&mut self, value: [u8; 32]) {
        self.hash = value;
    }

    pub fn set_type_id(&mut self, value: usize) {
        self.type_id = value;
    }

    pub fn set_cost(&mut self, value: usize) {
        self.cost = value;
    }

    pub fn set_subtypes(&mut self, value: HashSet<String>) {
        self.subtypes = value;
    }

    pub fn serialize_uri(&self) -> String {
        // const ConditionClass = TypeRegistry.findByTypeId(this.type).Class;
        // const includeSubtypes = ConditionClass.TYPE_CATEGORY === 'compound';
        // return 'ni:///sha-256;' +
        //   base64url.encode(this.getHash()) +
        //   '?fpt=' + this.getTypeName() +
        //   '&cost=' + this.getCost() +
        //   (includeSubtypes ? '&subtypes=' + Array.from(this.getSubtypes()).sort().join(',') : '')

        let hash = Base64Url::encode(&self.hash);
        let type_name = "ed25519-sha-256";
        let cost = self.cost;

        let uri = format!("ni:///sha-256;{hash}?fpt={type_name}&cost={cost}");

        uri
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_uri() {
        let hash = [1; 32];

        let condition = Condition {
            hash,
            type_id: 4,
            cost: 100,
            subtypes: HashSet::new(),
        };

        let uri = condition.serialize_uri();

        assert_eq!("ni:///sha-256;AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQE?fpt=ed25519-sha-256&cost=100", uri);
    }
}
