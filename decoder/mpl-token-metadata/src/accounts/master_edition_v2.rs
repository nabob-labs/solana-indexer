use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug)]
pub struct MasterEditionV2 {
    pub key: Key,
    pub supply: u64,
    pub max_supply: Option<u64>,
}
