use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1dc4f99421a8e44906")]
pub struct OpenOrdersPositionLogEvent {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub open_orders_account_num: u32,
    pub market: solana_sdk::pubkey::Pubkey,
    pub bids_base_lots: i64,
    pub bids_quote_lots: i64,
    pub asks_base_lots: i64,
    pub base_free_native: u64,
    pub quote_free_native: u64,
    pub locked_maker_fees: u64,
    pub referrer_rebates_available: u64,
    pub maker_volume: u128,
    pub taker_volume: u128,
}
