use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0xb6a1fc657ba1cdb8")]
pub struct InsuranceDepositAccount {
    pub nonce: u8,
    pub amount: u64,
}
