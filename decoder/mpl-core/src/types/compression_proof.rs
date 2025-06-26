use {
    super::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CompressionProof {
    pub owner: solana_pubkey::Pubkey,
    pub update_authority: UpdateAuthority,
    pub name: String,
    pub uri: String,
    pub seq: u64,
    pub plugins: Vec<HashablePluginSchema>,
}
