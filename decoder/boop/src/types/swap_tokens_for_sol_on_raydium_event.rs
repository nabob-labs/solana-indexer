use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SwapTokensForSolOnRaydiumEvent {
    pub mint: solana_pubkey::Pubkey,
    pub amount_in: u64,
    pub amount_out: u64,
}
