use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1da384d980f35c5af9")]
pub struct AuthorityTransferCompletedEvent {
    pub old_authority: solana_pubkey::Pubkey,
    pub new_authority: solana_pubkey::Pubkey,
}
