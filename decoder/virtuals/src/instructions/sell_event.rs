use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d3e2f370aa503dc2a")]
pub struct SellEvent {
    pub sell_amount: u64,
    pub virtuals_amount: u64,
}
