use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xde7a01dd7b748f6e")]
pub struct UpdateTickSize {
    pub asset: Asset,
    pub tick_size: u32,
}

pub struct UpdateTickSizeInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateTickSize {
    type ArrangedAccounts = UpdateTickSizeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateTickSizeInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
        })
    }
}
