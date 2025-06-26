use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x37e67dda952741f8")]
pub struct OrderBook {
    pub version: u8,
    pub order_book_type: OrderBookType,
    pub apy: APY,
    pub loan_terms: BookLoanTerms,
    pub fee_permillicentage: u16,
    pub fee_authority: solana_pubkey::Pubkey,
}
