use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug)]
pub struct ReservationListV2 {
    pub key: Key,
    pub master_edition: solana_pubkey::Pubkey,
    pub supply_snapshot: Option<u64>,
    pub reservations: Vec<Reservation>,
    pub total_reservation_spots: u64,
    pub current_reservation_spots: u64,
}
