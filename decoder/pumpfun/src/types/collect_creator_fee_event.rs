use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CollectCreatorFeeEvent {
    pub timestamp: i64,
    pub creator: solana_pubkey::Pubkey,
    pub creator_fee: u64,
}
