use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateAdminEvent {
    pub timestamp: i64,
    pub admin: solana_pubkey::Pubkey,
    pub new_admin: solana_pubkey::Pubkey,
}
