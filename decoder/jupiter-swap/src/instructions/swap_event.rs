use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d40c6cde8260871e2")]
pub struct SwapEvent {
    pub amm: solana_pubkey::Pubkey,
    pub input_mint: solana_pubkey::Pubkey,
    pub input_amount: u64,
    pub output_mint: solana_pubkey::Pubkey,
    pub output_amount: u64,
}
