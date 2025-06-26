use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1ddcb743d799cf38ea")]
pub struct LockEvent {
    pub pool: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub amount: u64,
}
