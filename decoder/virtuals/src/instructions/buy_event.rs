use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d67f4521f2cf57777")]
pub struct BuyEvent {
    pub buy_amount: u64,
    pub virtuals_amount: u64,
}
