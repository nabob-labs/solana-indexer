use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitializeLbPair2Params {
    pub active_id: i32,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u8; 96],
}
