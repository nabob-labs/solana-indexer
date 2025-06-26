use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TokenParams {
    pub decimals: u32,
    pub vault_bump: u32,
    pub mint_key: solana_pubkey::Pubkey,
    pub vault_key: solana_pubkey::Pubkey,
}
