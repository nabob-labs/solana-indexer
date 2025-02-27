use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x73b2c908afb77b77")]
pub struct CancelOrderByClientOrderId {
    pub client_order_id: u64,
    pub asset: Asset,
}

pub struct CancelOrderByClientOrderIdInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub cancel_accounts: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CancelOrderByClientOrderId {
    type ArrangedAccounts = CancelOrderByClientOrderIdInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, cancel_accounts, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CancelOrderByClientOrderIdInstructionAccounts {
            authority: authority.pubkey,
            cancel_accounts: cancel_accounts.pubkey,
        })
    }
}
