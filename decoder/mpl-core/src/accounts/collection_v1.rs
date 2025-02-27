use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0xf1e9caaec3d4e560")]
pub struct CollectionV1 {
    pub key: Key,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub name: String,
    pub uri: String,
    pub num_minted: u32,
    pub current_size: u32,
}
