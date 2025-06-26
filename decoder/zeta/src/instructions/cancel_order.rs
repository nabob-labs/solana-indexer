use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x5f81edf00831df84")]
pub struct CancelOrder {
    pub side: Side,
    pub order_id: u128,
    pub asset: Asset,
}

pub struct CancelOrderInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub cancel_accounts: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CancelOrder {
    type ArrangedAccounts = CancelOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, cancel_accounts, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CancelOrderInstructionAccounts {
            authority: authority.pubkey,
            cancel_accounts: cancel_accounts.pubkey,
        })
    }
}
