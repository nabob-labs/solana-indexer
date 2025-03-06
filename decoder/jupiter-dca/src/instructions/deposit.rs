use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d3ecdf2aff4a98834")]
pub struct Deposit {
    pub dca_key: solana_sdk::pubkey::Pubkey,
    pub amount: u64,
}
