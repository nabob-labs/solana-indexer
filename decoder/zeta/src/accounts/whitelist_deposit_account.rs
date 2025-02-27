use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x6e02d95144ae78d9")]
pub struct WhitelistDepositAccount {
    pub nonce: u8,
    pub user_key: solana_sdk::pubkey::Pubkey,
}
