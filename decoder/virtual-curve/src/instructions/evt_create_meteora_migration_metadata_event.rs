use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d63a7853fd68faf8b")]
pub struct EvtCreateMeteoraMigrationMetadataEvent {
    pub virtual_pool: solana_pubkey::Pubkey,
}
