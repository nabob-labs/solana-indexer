use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x85dcadd5b3d32bee")]
pub struct MarginAccount {
    pub authority: solana_pubkey::Pubkey,
    pub nonce: u8,
    pub balance: u64,
    pub force_cancel_flag: bool,
    pub open_orders_nonce: [u8; 138],
    pub series_expiry: [u64; 5],
    pub series_expiry_padding: u64,
    pub product_ledgers: [ProductLedger; 46],
    pub product_ledgers_padding: [ProductLedger; 91],
    pub perp_product_ledger: ProductLedger,
    pub rebalance_amount: i64,
    pub asset: Asset,
    pub account_type: MarginAccountType,
    pub last_funding_delta: AnchorDecimal,
    pub delegated_pubkey: solana_pubkey::Pubkey,
    pub rebate_rebalance_amount: u64,
    pub padding: [u8; 330],
}
