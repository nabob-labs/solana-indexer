use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TokenMultiplier {
    pub token_a_multiplier: u64,
    pub token_b_multiplier: u64,
    pub precision_factor: u8,
}
