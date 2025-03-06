use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
pub struct ReservationListV1 {
    pub key: Key,
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub supply_snapshot: Option<u64>,
    pub reservations: Vec<ReservationV1>,
}
