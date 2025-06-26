use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct LiquidationBalances {
    pub liquidatee_asset_balance: f64,
    pub liquidatee_liability_balance: f64,
    pub liquidator_asset_balance: f64,
    pub liquidator_liability_balance: f64,
}
