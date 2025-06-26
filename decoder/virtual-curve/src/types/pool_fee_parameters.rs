use super::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PoolFeeParameters {
    pub base_fee: BaseFeeParameters,
    pub dynamic_fee: Option<DynamicFeeParameters>,
}
