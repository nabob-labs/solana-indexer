use {
    solana_indexer_core::{borsh, IndexerDeserialize},
    serde_big_array::BigArray,
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AnyNode {
    pub tag: u8,
    #[serde(with = "BigArray")]
    pub data: [u8; 87],
}
