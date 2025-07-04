use super::super::types::*;

use alloc::vec::Vec;
use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1dee081207a23b6891")]
pub struct PlaceMultiOrdersEvent {
    pub oracle_price: u64,
    pub order_ids: Vec<u128>,
    pub expiry_tss: Vec<u64>,
    pub asset: Asset,
    pub margin_account: solana_pubkey::Pubkey,
    pub client_order_ids: Vec<u64>,
    pub user: solana_pubkey::Pubkey,
}
