use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d241eef2d3a840e05")]
pub struct EvtCloseConfigEvent {
    pub config: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}
