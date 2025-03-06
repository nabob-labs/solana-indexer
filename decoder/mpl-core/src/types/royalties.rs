
use super::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Royalties {
    pub basis_points: u16,
    pub creators: Vec<Creator>,
    pub rule_set: RuleSet,
}
