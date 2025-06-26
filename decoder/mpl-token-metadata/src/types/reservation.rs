use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Reservation {
    pub address: solana_pubkey::Pubkey,
    pub spots_remaining: u64,
    pub total_spots: u64,
}
