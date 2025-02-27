use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x1f489fe8dc995a6d")]
pub struct UpdateOrderBook {
    pub order_book_type: Option<OrderBookType>,
    pub apy: Option<APY>,
    pub loan_terms: Option<BookLoanTerms>,
    pub fee_permillicentage: Option<u16>,
    pub fee_authority: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct UpdateOrderBookInstructionAccounts {
    pub order_book: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateOrderBook {
    type ArrangedAccounts = UpdateOrderBookInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [order_book, payer, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateOrderBookInstructionAccounts {
            order_book: order_book.pubkey,
            payer: payer.pubkey,
        })
    }
}
