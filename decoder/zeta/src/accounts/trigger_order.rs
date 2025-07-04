use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0xec3d2abe980c6a74")]
pub struct TriggerOrder {
    pub owner: solana_pubkey::Pubkey,
    pub margin_account: solana_pubkey::Pubkey,
    pub open_orders: solana_pubkey::Pubkey,
    pub order_price: u64,
    pub trigger_price: Option<u64>,
    pub trigger_ts: Option<u64>,
    pub size: u64,
    pub creation_ts: u64,
    pub trigger_direction: Option<TriggerDirection>,
    pub side: Side,
    pub asset: Asset,
    pub order_type: OrderType,
    pub bit: u8,
    pub reduce_only: bool,
}
