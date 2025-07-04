use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1de43f9bf984a08771")]
pub struct OrderUnstakeEvent {
    pub state: solana_pubkey::Pubkey,
    pub ticket_epoch: u64,
    pub ticket: solana_pubkey::Pubkey,
    pub beneficiary: solana_pubkey::Pubkey,
    pub circulating_ticket_balance: u64,
    pub circulating_ticket_count: u64,
    pub user_msol_balance: u64,
    pub burned_msol_amount: u64,
    pub sol_amount: u64,
    pub fee_bp_cents: u32,
    pub total_virtual_staked_lamports: u64,
    pub msol_supply: u64,
}
