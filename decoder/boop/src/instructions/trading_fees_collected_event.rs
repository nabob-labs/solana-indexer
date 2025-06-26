use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1de13f1a3786f3d2cb")]
pub struct TradingFeesCollectedEvent {
    pub pool_state: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
}
