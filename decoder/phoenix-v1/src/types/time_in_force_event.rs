use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TimeInForceEvent {
    pub index: u16,
    pub order_sequence_number: u64,
    pub last_valid_slot: u64,
    pub last_valid_unix_timestamp_in_seconds: u64,
}
