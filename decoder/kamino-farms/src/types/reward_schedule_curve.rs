use {
    super::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RewardScheduleCurve {
    pub points: [RewardPerTimeUnitPoint; 20],
}
