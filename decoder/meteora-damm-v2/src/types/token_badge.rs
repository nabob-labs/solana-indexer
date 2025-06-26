use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TokenBadge {
    pub token_mint: solana_pubkey::Pubkey,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u8; 128],
}
