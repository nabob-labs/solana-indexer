use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1de7cb76604b7446e4")]
pub struct DepositStakeAccountEvent {
    pub state: solana_pubkey::Pubkey,
    pub stake: solana_pubkey::Pubkey,
    pub delegated: u64,
    pub withdrawer: solana_pubkey::Pubkey,
    pub stake_index: u32,
    pub validator: solana_pubkey::Pubkey,
    pub validator_index: u32,
    pub validator_active_balance: u64,
    pub total_active_balance: u64,
    pub user_msol_balance: u64,
    pub msol_minted: u64,
    pub total_virtual_staked_lamports: u64,
    pub msol_supply: u64,
}
