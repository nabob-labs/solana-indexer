use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x8bbee6f94da0ce04")]
pub struct CancelAllMarketOrders {
    pub asset: Asset,
}

pub struct CancelAllMarketOrdersInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub cancel_accounts: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CancelAllMarketOrders {
    type ArrangedAccounts = CancelAllMarketOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, cancel_accounts, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CancelAllMarketOrdersInstructionAccounts {
            authority: authority.pubkey,
            cancel_accounts: cancel_accounts.pubkey,
        })
    }
}
