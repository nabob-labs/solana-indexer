use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d9dca235ca5a32738")]
pub struct TokenCreatedFallbackEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
