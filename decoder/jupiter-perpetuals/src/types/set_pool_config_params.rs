use super::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SetPoolConfigParams {
    pub fees: Fees,
    pub limit: Limit,
    pub max_request_execution_sec: i64,
}
