use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1de7bd419566d79af4")]
pub struct WithdrawIneligibleRewardEvent {
    pub lb_pair: solana_pubkey::Pubkey,
    pub reward_mint: solana_pubkey::Pubkey,
    pub amount: u64,
}
