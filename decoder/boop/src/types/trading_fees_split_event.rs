use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TradingFeesSplitEvent {
    pub amount: u64,
    pub creator: solana_pubkey::Pubkey,
}
