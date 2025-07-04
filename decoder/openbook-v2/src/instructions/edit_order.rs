use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xfed0761dadf8c846")]
pub struct EditOrder {
    pub client_order_id: u64,
    pub expected_cancel_size: i64,
    pub place_order: PlaceOrderArgs,
}

pub struct EditOrderInstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub open_orders_account: solana_pubkey::Pubkey,
    pub open_orders_admin: solana_pubkey::Pubkey,
    pub user_token_account: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub bids: solana_pubkey::Pubkey,
    pub asks: solana_pubkey::Pubkey,
    pub event_heap: solana_pubkey::Pubkey,
    pub market_vault: solana_pubkey::Pubkey,
    pub oracle_a: solana_pubkey::Pubkey,
    pub oracle_b: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for EditOrder {
    type ArrangedAccounts = EditOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [signer, open_orders_account, open_orders_admin, user_token_account, market, bids, asks, event_heap, market_vault, oracle_a, oracle_b, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(EditOrderInstructionAccounts {
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
