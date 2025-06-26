use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d4c9ad00d2873f692")]
pub struct EvtUpdateRewardFunderEvent {
    pub pool: solana_pubkey::Pubkey,
    pub reward_index: u8,
    pub old_funder: solana_pubkey::Pubkey,
    pub new_funder: solana_pubkey::Pubkey,
}
