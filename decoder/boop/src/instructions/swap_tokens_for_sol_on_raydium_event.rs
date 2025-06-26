use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d4cf9dda241467620")]
pub struct SwapTokensForSolOnRaydiumEvent {
    pub mint: solana_pubkey::Pubkey,
    pub amount_in: u64,
    pub amount_out: u64,
}
