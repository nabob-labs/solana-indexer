use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0xc56d2e767fef7e32")]
pub struct HashedAssetV1 {
    pub key: Key,
    pub hash: [u8; 32],
}
