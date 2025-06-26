use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0xffc24e7b1069d0a5")]
pub struct OpenOrdersAccount {
    pub owner: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub name: [u8; 32],
    pub delegate: NonZeroPubkeyOption,
    pub account_num: u32,
    pub bump: u8,
    pub version: u8,
    pub padding: [u8; 2],
    pub position: Position,
    pub open_orders: [OpenOrder; 24],
}
