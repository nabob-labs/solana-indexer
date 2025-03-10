use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreateEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub bonding_curve: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
}
