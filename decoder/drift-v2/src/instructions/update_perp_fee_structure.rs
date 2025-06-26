use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x17b26fcb49168c4b")]
pub struct UpdatePerpFeeStructure {
    pub fee_structure: FeeStructure,
}

pub struct UpdatePerpFeeStructureInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdatePerpFeeStructure {
    type ArrangedAccounts = UpdatePerpFeeStructureInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdatePerpFeeStructureInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
