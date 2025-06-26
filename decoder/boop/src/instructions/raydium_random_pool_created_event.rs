use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d98fb80989eeb5335")]
pub struct RaydiumRandomPoolCreatedEvent {
    pub pool_state: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
}
