use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum PerpOperation {
    UpdateFunding,
    AmmFill,
    Fill,
    SettlePnl,
    SettlePnlWithPosition,
    Liquidation,
    AmmImmediateFill,
}
