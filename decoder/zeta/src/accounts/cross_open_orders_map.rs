use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0xc5185209520e309a")]
pub struct CrossOpenOrdersMap {
    pub user_key: solana_pubkey::Pubkey,
    pub subaccount_index: u8,
}
