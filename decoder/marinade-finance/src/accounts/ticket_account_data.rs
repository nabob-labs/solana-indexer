use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x854d1262d301e703")]
pub struct TicketAccountData {
    pub state_address: solana_pubkey::Pubkey,
    pub beneficiary: solana_pubkey::Pubkey,
    pub lamports_amount: u64,
    pub created_epoch: u64,
}
