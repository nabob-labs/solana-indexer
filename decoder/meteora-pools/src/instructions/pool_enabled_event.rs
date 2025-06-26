use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d02971253cc865cbf")]
pub struct PoolEnabledEvent {
    pub pool: solana_pubkey::Pubkey,
    pub enabled: bool,
}
