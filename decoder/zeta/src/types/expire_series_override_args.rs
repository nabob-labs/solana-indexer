use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ExpireSeriesOverrideArgs {
    pub settlement_nonce: u8,
    pub settlement_price: u64,
}
