use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1df14b87adccd74843")]
pub struct RedelegateEvent {
    pub state: solana_pubkey::Pubkey,
    pub epoch: u64,
    pub stake_index: u32,
    pub stake_account: solana_pubkey::Pubkey,
    pub last_update_delegation: u64,
    pub source_validator_index: u32,
    pub source_validator_vote: solana_pubkey::Pubkey,
    pub source_validator_score: u32,
    pub source_validator_balance: u64,
    pub source_validator_stake_target: u64,
    pub dest_validator_index: u32,
    pub dest_validator_vote: solana_pubkey::Pubkey,
    pub dest_validator_score: u32,
    pub dest_validator_balance: u64,
    pub dest_validator_stake_target: u64,
    pub redelegate_amount: u64,
    pub split_stake_account: Option<SplitStakeAccountInfo>,
    pub redelegate_stake_index: u32,
    pub redelegate_stake_account: solana_pubkey::Pubkey,
}
