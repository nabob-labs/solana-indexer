use {
    super::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
    std::hash::{Hash, Hasher},
};

#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct Payload {
    pub map: std::collections::HashMap<String, PayloadType>,
}

impl Hash for Payload {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (key, value) in &self.map {
            key.hash(state);
            value.hash(state);
        }
    }
}
