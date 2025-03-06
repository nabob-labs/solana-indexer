

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ReserveFees {
    pub borrow_fee_sf: u64,
    pub flash_loan_fee_sf: u64,
    pub padding: [u8; 8],
}
