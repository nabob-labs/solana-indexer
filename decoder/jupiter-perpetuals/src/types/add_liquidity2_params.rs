use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AddLiquidity2Params {
    pub token_amount_in: u64,
    pub min_lp_amount_out: u64,
    pub token_amount_pre_swap: Option<u64>,
}
