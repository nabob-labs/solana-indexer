use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1db6c3892a23cecff7")]
pub struct UpdateGlobalAuthorityEvent {
    pub global: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub new_authority: solana_pubkey::Pubkey,
    pub timestamp: i64,
}
