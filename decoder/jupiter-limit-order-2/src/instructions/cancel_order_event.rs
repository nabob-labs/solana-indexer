use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1dae428d1104e0a24d")]
pub struct CancelOrderEvent {
    pub order_key: solana_sdk::pubkey::Pubkey,
}
