use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x61d8698371f68e8d")]
pub struct UpdateSpotFeeStructure {
    pub fee_structure: FeeStructure,
}

pub struct UpdateSpotFeeStructureInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateSpotFeeStructure {
    type ArrangedAccounts = UpdateSpotFeeStructureInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateSpotFeeStructureInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
