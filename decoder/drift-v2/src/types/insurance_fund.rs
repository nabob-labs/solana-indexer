use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InsuranceFund {
    pub vault: solana_pubkey::Pubkey,
    pub total_shares: u128,
    pub user_shares: u128,
    pub shares_base: u128,
    pub unstaking_period: i64,
    pub last_revenue_settle_ts: i64,
    pub revenue_settle_period: i64,
    pub total_factor: u32,
    pub user_factor: u32,
}
