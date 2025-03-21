use solana_indexer_core::{borsh, IndexerDeserialize};

#[repr(u8)]
#[derive(
    IndexerDeserialize,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Clone,
    Hash,
    Default,
)]
pub enum CurveType {
    LinearV1,
    #[default]
    ConstantProductV1,
}
