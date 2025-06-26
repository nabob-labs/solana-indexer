use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum SpotFulfillmentMethod {
    ExternalMarket,
    Match(solana_pubkey::Pubkey, u16),
}
