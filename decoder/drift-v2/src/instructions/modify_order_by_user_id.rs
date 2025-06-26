use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x9e4d04fdfcc2a1b3")]
pub struct ModifyOrderByUserId {
    pub user_order_id: u8,
    pub modify_order_params: ModifyOrderParams,
}

pub struct ModifyOrderByUserIdInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ModifyOrderByUserId {
    type ArrangedAccounts = ModifyOrderByUserIdInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, user, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ModifyOrderByUserIdInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
            authority: authority.pubkey,
        })
    }
}
