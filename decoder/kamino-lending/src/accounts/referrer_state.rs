use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0xc251d9670c130c42")]
pub struct ReferrerState {
    pub short_url: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
}
