use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MasterEdition {
    pub max_supply: Option<u32>,
    pub name: Option<String>,
    pub uri: Option<String>,
}
