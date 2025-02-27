use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0xfa7eac0a761e03a8")]
pub struct OpenOrdersMap {
    pub user_key: solana_sdk::pubkey::Pubkey,
}
