use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d676f84a88cfd9672")]
pub struct EvtCreateDammV2MigrationMetadataEvent {
    pub virtual_pool: solana_pubkey::Pubkey,
}
