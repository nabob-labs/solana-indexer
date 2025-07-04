use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d79760b0bc6428e73")]
pub struct PoolSwapExactOutEvent {
    pub receiving_custody_key: solana_pubkey::Pubkey,
    pub dispensing_custody_key: solana_pubkey::Pubkey,
    pub pool_key: solana_pubkey::Pubkey,
    pub amount_in: u64,
    pub amount_in_after_fees: u64,
    pub amount_out: u64,
    pub swap_usd_amount: u64,
    pub fee_bps: u64,
    pub owner_key: solana_pubkey::Pubkey,
    pub receiving_account_key: solana_pubkey::Pubkey,
}
