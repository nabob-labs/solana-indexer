use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d499c45e9200e9641")]
pub struct MergeStakesEvent {
    pub state: solana_pubkey::Pubkey,
    pub epoch: u64,
    pub destination_stake_index: u32,
    pub destination_stake_account: solana_pubkey::Pubkey,
    pub last_update_destination_stake_delegation: u64,
    pub source_stake_index: u32,
    pub source_stake_account: solana_pubkey::Pubkey,
    pub last_update_source_stake_delegation: u64,
    pub validator_index: u32,
    pub validator_vote: solana_pubkey::Pubkey,
    pub extra_delegated: u64,
    pub returned_stake_rent: u64,
    pub validator_active_balance: u64,
    pub total_active_balance: u64,
    pub operational_sol_balance: u64,
}
