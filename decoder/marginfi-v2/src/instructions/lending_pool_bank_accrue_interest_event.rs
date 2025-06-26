use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
#[indexer(discriminator = "0xe445a52e51cb9a1d6875bb9c6f9a6aba")]
pub struct LendingPoolBankAccrueInterestEvent {
    pub header: GroupEventHeader,
    pub bank: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub delta: u64,
    pub fees_collected: f64,
    pub insurance_collected: f64,
}
