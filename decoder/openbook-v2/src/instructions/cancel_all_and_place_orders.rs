use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x809bde3cba28e132")]
pub struct CancelAllAndPlaceOrders {
    pub orders_type: PlaceOrderType,
    pub bids: Vec<PlaceMultipleOrdersArgs>,
    pub asks: Vec<PlaceMultipleOrdersArgs>,
    pub limit: u8,
}

pub struct CancelAllAndPlaceOrdersInstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub open_orders_account: solana_pubkey::Pubkey,
    pub open_orders_admin: solana_pubkey::Pubkey,
    pub user_quote_account: solana_pubkey::Pubkey,
    pub user_base_account: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub bids: solana_pubkey::Pubkey,
    pub asks: solana_pubkey::Pubkey,
    pub event_heap: solana_pubkey::Pubkey,
    pub market_quote_vault: solana_pubkey::Pubkey,
    pub market_base_vault: solana_pubkey::Pubkey,
    pub oracle_a: solana_pubkey::Pubkey,
    pub oracle_b: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CancelAllAndPlaceOrders {
    type ArrangedAccounts = CancelAllAndPlaceOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [signer, open_orders_account, open_orders_admin, user_quote_account, user_base_account, market, bids, asks, event_heap, market_quote_vault, market_base_vault, oracle_a, oracle_b, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CancelAllAndPlaceOrdersInstructionAccounts {
            signer: signer.pubkey,
            open_orders_account: open_orders_account.pubkey,
            open_orders_admin: open_orders_admin.pubkey,
            user_quote_account: user_quote_account.pubkey,
            user_base_account: user_base_account.pubkey,
            market: market.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
            event_heap: event_heap.pubkey,
            market_quote_vault: market_quote_vault.pubkey,
            market_base_vault: market_base_vault.pubkey,
            oracle_a: oracle_a.pubkey,
            oracle_b: oracle_b.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
