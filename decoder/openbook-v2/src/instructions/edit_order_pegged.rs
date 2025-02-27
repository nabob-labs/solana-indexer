use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x3ebb7d451add9d85")]
pub struct EditOrderPegged {
    pub client_order_id: u64,
    pub expected_cancel_size: i64,
    pub place_order: PlaceOrderPeggedArgs,
}

pub struct EditOrderPeggedInstructionAccounts {
    pub signer: solana_sdk::pubkey::Pubkey,
    pub open_orders_account: solana_sdk::pubkey::Pubkey,
    pub open_orders_admin: solana_sdk::pubkey::Pubkey,
    pub user_token_account: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub bids: solana_sdk::pubkey::Pubkey,
    pub asks: solana_sdk::pubkey::Pubkey,
    pub event_heap: solana_sdk::pubkey::Pubkey,
    pub market_vault: solana_sdk::pubkey::Pubkey,
    pub oracle_a: solana_sdk::pubkey::Pubkey,
    pub oracle_b: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for EditOrderPegged {
    type ArrangedAccounts = EditOrderPeggedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [signer, open_orders_account, open_orders_admin, user_token_account, market, bids, asks, event_heap, market_vault, oracle_a, oracle_b, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(EditOrderPeggedInstructionAccounts {
            signer: signer.pubkey,
            open_orders_account: open_orders_account.pubkey,
            open_orders_admin: open_orders_admin.pubkey,
            user_token_account: user_token_account.pubkey,
            market: market.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
            event_heap: event_heap.pubkey,
            market_vault: market_vault.pubkey,
            oracle_a: oracle_a.pubkey,
            oracle_b: oracle_b.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
