use {
    super::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct WithdrawParams {
    pub withdraw_amount: u64,
    pub withdrawal: Withdrawal,
}
