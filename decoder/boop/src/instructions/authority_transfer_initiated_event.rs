use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d79f65f9be56d94cd")]
pub struct AuthorityTransferInitiatedEvent {
    pub old_authority: solana_pubkey::Pubkey,
    pub new_authority: solana_pubkey::Pubkey,
}
