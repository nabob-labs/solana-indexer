use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1dfc9fb193b671ba5e")]
pub struct UpdateDeactivatedEvent {
    pub state: solana_pubkey::Pubkey,
    pub epoch: u64,
    pub stake_index: u32,
    pub stake_account: solana_pubkey::Pubkey,
    pub balance_without_rent_exempt: u64,
    pub last_update_delegated_lamports: u64,
    pub msol_fees: Option<u64>,
    pub msol_price_change: U64ValueChange,
    pub reward_fee_used: Fee,
    pub operational_sol_balance: u64,
    pub total_virtual_staked_lamports: u64,
    pub msol_supply: u64,
}
