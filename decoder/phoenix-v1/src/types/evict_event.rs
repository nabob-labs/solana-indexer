use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvictEvent {
    pub index: u16,
    pub maker_id: solana_pubkey::Pubkey,
    pub order_sequence_number: u64,
    pub price_in_ticks: u64,
    pub base_lots_evicted: u64,
}
