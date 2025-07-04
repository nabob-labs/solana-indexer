use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xf2e7b1fb7e919f68")]
pub struct SetTestTime {
    pub params: SetTestTimeParams,
}

pub struct SetTestTimeInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub perpetuals: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetTestTime {
    type ArrangedAccounts = SetTestTimeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, perpetuals, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetTestTimeInstructionAccounts {
            admin: admin.pubkey,
            perpetuals: perpetuals.pubkey,
        })
    }
}
