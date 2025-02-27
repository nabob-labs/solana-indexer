use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateMarginParametersArgs {
    pub future_margin_initial: u64,
    pub future_margin_maintenance: u64,
}
