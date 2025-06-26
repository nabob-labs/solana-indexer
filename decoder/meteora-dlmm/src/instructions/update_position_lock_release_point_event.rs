use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d85d642e0400c07bf")]
pub struct UpdatePositionLockReleasePointEvent {
    pub position: solana_pubkey::Pubkey,
    pub current_point: u64,
    pub new_lock_release_point: u64,
    pub old_lock_release_point: u64,
    pub sender: solana_pubkey::Pubkey,
}
