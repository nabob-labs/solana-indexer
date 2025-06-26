use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SetMetaplexCreatorEvent {
    pub timestamp: i64,
    pub mint: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
}
