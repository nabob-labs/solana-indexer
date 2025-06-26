use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d4a5e6a8d3111626d")]
pub struct CreateLockEscrowEvent {
    pub pool: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
}
