use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize)]
#[indexer(discriminator = "0x5c8e5cdc059446b5")]
pub struct BinArray {
    pub index: i64,
    pub version: u8,
    pub padding: [u8; 7],
    pub lb_pair: solana_pubkey::Pubkey,
    #[serde(with = "serde_big_array::BigArray")]
    pub bins: [Bin; 70],
}
