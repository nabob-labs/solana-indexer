use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SwapInstructionBaseIn {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}
