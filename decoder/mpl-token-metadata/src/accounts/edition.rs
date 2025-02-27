use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug)]
pub struct Edition {
    pub key: Key,
    pub parent: solana_sdk::pubkey::Pubkey,
    pub edition: u64,
}
