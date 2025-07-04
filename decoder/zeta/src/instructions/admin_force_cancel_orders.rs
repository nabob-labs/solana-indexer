use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x43347cc0bf20da5b")]
pub struct AdminForceCancelOrders {
    pub asset: Asset,
}

pub struct AdminForceCancelOrdersInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub cancel_accounts: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for AdminForceCancelOrders {
    type ArrangedAccounts = AdminForceCancelOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, cancel_accounts, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AdminForceCancelOrdersInstructionAccounts {
            authority: authority.pubkey,
            cancel_accounts: cancel_accounts.pubkey,
        })
    }
}
