use super::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct OracleParams {
    pub oracle_account: solana_pubkey::Pubkey,
    pub oracle_type: OracleType,
    pub max_price_error: u64,
    pub max_price_age_sec: u32,
}
