use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1df9b56c5904965aae")]
pub struct CloseConfigEvent {
    pub config: solana_pubkey::Pubkey,
}
