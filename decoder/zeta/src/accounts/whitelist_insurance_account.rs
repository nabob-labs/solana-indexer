use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x0a68c0cb813c2802")]
pub struct WhitelistInsuranceAccount {
    pub nonce: u8,
    pub user_key: solana_sdk::pubkey::Pubkey,
}
