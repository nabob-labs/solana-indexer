use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1df6e43a8291aa4fcc")]
pub struct FundRewardEvent {
    pub lb_pair: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub reward_index: u64,
    pub amount: u64,
}
