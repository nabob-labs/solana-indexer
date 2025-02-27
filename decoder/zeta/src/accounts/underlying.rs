use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0xce80984d70a40d02")]
pub struct Underlying {
    pub mint: solana_sdk::pubkey::Pubkey,
}
