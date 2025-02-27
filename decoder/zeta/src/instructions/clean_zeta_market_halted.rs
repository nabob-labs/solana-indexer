use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x898c5e12e7e8d9cc")]
pub struct CleanZetaMarketHalted {
    pub asset: Asset,
}

pub struct CleanZetaMarketHaltedInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub bids: solana_sdk::pubkey::Pubkey,
    pub asks: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CleanZetaMarketHalted {
    type ArrangedAccounts = CleanZetaMarketHaltedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, market, bids, asks, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CleanZetaMarketHaltedInstructionAccounts {
            state: state.pubkey,
            market: market.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
        })
    }
}
