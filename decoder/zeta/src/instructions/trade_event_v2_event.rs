use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d128ed28bb27bcb75")]
pub struct TradeEventV2Event {
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub index: u8,
    pub size: u64,
    pub cost_of_trades: u64,
    pub is_bid: bool,
    pub client_order_id: u64,
    pub order_id: u128,
    pub asset: u8,
    pub user: solana_sdk::pubkey::Pubkey,
    pub is_taker: bool,
    pub sequence_number: u64,
}
