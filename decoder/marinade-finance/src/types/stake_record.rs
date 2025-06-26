use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct StakeRecord {
    pub stake_account: solana_pubkey::Pubkey,
    pub last_update_delegated_lamports: u64,
    pub last_update_epoch: u64,
    pub is_emergency_unstaking: u8,
}
