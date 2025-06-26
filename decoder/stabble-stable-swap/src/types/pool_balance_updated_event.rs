use super::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PoolBalanceUpdatedEvent {
    pub pubkey: solana_pubkey::Pubkey,
    pub data: PoolBalanceUpdatedData,
}
