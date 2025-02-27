use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x589af8ba300e7bf4")]
pub struct CloseMarket {}

pub struct CloseMarketInstructionAccounts {
    pub close_market_admin: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub bids: solana_sdk::pubkey::Pubkey,
    pub asks: solana_sdk::pubkey::Pubkey,
    pub event_heap: solana_sdk::pubkey::Pubkey,
    pub sol_destination: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CloseMarket {
    type ArrangedAccounts = CloseMarketInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [close_market_admin, market, bids, asks, event_heap, sol_destination, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CloseMarketInstructionAccounts {
            close_market_admin: close_market_admin.pubkey,
            market: market.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
            event_heap: event_heap.pubkey,
            sol_destination: sol_destination.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
