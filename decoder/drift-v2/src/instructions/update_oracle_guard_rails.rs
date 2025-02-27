use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x83700a3b203628a4")]
pub struct UpdateOracleGuardRails {
    pub oracle_guard_rails: OracleGuardRails,
}

pub struct UpdateOracleGuardRailsInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateOracleGuardRails {
    type ArrangedAccounts = UpdateOracleGuardRailsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateOracleGuardRailsInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
