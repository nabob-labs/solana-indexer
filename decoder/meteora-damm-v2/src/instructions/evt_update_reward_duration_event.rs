use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d958741e781994139")]
pub struct EvtUpdateRewardDurationEvent {
    pub pool: solana_pubkey::Pubkey,
    pub reward_index: u8,
    pub old_reward_duration: u64,
    pub new_reward_duration: u64,
}
