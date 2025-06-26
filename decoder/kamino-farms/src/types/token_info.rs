use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TokenInfo {
    pub mint: solana_pubkey::Pubkey,
    pub decimals: u64,
    pub token_program: solana_pubkey::Pubkey,
    pub padding: [u64; 6],
}
