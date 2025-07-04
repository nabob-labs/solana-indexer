use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d5d0f46aa308cd4db")]
pub struct ClaimEvent {
    pub state: solana_pubkey::Pubkey,
    pub epoch: u64,
    pub ticket: solana_pubkey::Pubkey,
    pub beneficiary: solana_pubkey::Pubkey,
    pub circulating_ticket_balance: u64,
    pub circulating_ticket_count: u64,
    pub reserve_balance: u64,
    pub user_balance: u64,
    pub amount: u64,
}
