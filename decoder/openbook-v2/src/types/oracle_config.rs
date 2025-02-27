use {
    solana_indexer_core::{borsh, IndexerDeserialize},
    serde_big_array::BigArray,
};

#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct OracleConfig {
    pub conf_filter: f64,
    pub max_staleness_slots: i64,
    #[serde(with = "BigArray")]
    pub reserved: [u8; 72],
}
