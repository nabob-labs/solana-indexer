use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x00c0e902fcfb82a9")]
pub struct CancelOrderHalted {
    pub side: Side,
    pub order_id: u128,
    pub asset: Asset,
}

pub struct CancelOrderHaltedInstructionAccounts {
    pub cancel_accounts: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CancelOrderHalted {
    type ArrangedAccounts = CancelOrderHaltedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [cancel_accounts, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CancelOrderHaltedInstructionAccounts {
            cancel_accounts: cancel_accounts.pubkey,
        })
    }
}
