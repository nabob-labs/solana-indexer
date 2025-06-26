use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TokenCreatedFallbackEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
