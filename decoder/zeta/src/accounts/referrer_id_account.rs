use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0xcfc24e8a9e4aba7f")]
pub struct ReferrerIdAccount {
    pub referrer_id: [u8; 6],
    pub referrer_pubkey: solana_sdk::pubkey::Pubkey,
}
