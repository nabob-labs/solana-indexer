use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d607a718a32e39539")]
pub struct TokenCreatedEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
