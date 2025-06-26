use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1df73a7038cbba7098")]
pub struct OperatorsAddedEvent {
    pub operators: Vec<solana_pubkey::Pubkey>,
}
