use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PendingSharesToVest {
    pub deposit_slot: u64,
    pub lp_shares_to_vest: u64,
}
