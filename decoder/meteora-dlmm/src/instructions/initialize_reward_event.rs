use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1dd399583e953cb146")]
pub struct InitializeRewardEvent {
    pub lb_pair: solana_pubkey::Pubkey,
    pub reward_mint: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub reward_index: u64,
    pub reward_duration: u64,
}
