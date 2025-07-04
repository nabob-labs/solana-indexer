use {
    super::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitializeData {
    pub admin_authority: solana_pubkey::Pubkey,
    pub validator_manager_authority: solana_pubkey::Pubkey,
    pub min_stake: u64,
    pub rewards_fee: Fee,
    pub liq_pool: LiqPoolInitializeData,
    pub additional_stake_record_space: u32,
    pub additional_validator_record_space: u32,
    pub slots_for_stake_delta: u64,
    pub pause_authority: solana_pubkey::Pubkey,
}
