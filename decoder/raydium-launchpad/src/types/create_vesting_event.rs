use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreateVestingEvent {
    pub pool_state: solana_pubkey::Pubkey,
    pub beneficiary: solana_pubkey::Pubkey,
    pub share_amount: u64,
}
