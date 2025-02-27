use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d6d7fba4e724125ec")]
pub struct UpdateRewardInfosEvent {
    pub reward_growth_global_x64: [u128; 3],
}
