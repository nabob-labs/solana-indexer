use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x9cf709bc366c554d")]
pub struct TokenLedger {
    pub token_account: solana_pubkey::Pubkey,
    pub amount: u64,
}
