use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d83ee27301e1ba51c")]
pub struct WithdrawStakeAccountEvent {
    pub state: solana_pubkey::Pubkey,
    pub epoch: u64,
    pub stake: solana_pubkey::Pubkey,
    pub last_update_stake_delegation: u64,
    pub stake_index: u32,
    pub validator: solana_pubkey::Pubkey,
    pub validator_index: u32,
    pub user_msol_balance: u64,
    pub user_msol_auth: solana_pubkey::Pubkey,
    pub msol_burned: u64,
    pub msol_fees: u64,
    pub split_stake: solana_pubkey::Pubkey,
    pub beneficiary: solana_pubkey::Pubkey,
    pub split_lamports: u64,
    pub fee_bp_cents: u32,
    pub total_virtual_staked_lamports: u64,
    pub msol_supply: u64,
}
