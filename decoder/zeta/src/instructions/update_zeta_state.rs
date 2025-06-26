use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x68b614bb03a43c03")]
pub struct UpdateZetaState {
    pub args: UpdateStateArgs,
}

pub struct UpdateZetaStateInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateZetaState {
    type ArrangedAccounts = UpdateZetaStateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateZetaStateInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
        })
    }
}
