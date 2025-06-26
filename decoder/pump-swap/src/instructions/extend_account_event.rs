use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d6161d7905d92167c")]
pub struct ExtendAccountEvent {
    pub timestamp: i64,
    pub account: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub current_size: u64,
    pub new_size: u64,
}
