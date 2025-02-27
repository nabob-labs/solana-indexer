use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PoolToken {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub decimals: u8,
    pub scaling_up: bool,
    pub scaling_factor: u64,
    pub balance: u64,
}
