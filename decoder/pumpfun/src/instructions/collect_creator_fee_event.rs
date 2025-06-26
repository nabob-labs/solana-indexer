use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d7a027f010ebf0caf")]
pub struct CollectCreatorFeeEvent {
    pub timestamp: i64,
    pub creator: solana_pubkey::Pubkey,
    pub creator_fee: u64,
}
