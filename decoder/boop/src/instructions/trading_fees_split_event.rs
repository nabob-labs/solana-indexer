use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d713c9f11fdae877a")]
pub struct TradingFeesSplitEvent {
    pub amount: u64,
    pub creator: solana_pubkey::Pubkey,
}
