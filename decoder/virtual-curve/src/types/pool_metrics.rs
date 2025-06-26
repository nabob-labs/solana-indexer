use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PoolMetrics {
    pub total_protocol_base_fee: u64,
    pub total_protocol_quote_fee: u64,
    pub total_trading_base_fee: u64,
    pub total_trading_quote_fee: u64,
}
