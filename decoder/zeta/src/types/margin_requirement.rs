use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum MarginRequirement {
    Initial,
    Maintenance,
    MaintenanceIncludingOrders,
    MarketMakerConcession,
}
