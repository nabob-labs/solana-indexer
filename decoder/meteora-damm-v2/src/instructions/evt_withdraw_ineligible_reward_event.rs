use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1df8d7b84e1fb4b3a8")]
pub struct EvtWithdrawIneligibleRewardEvent {
    pub pool: solana_pubkey::Pubkey,
    pub reward_mint: solana_pubkey::Pubkey,
    pub amount: u64,
}
