use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0xdb27bda689f354ef")]
pub struct WhitelistTradingFeesAccount {
    pub nonce: u8,
    pub user_key: solana_pubkey::Pubkey,
}
