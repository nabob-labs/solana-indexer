use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x81c575726c77cf88")]
pub struct OverrideExpiry {
    pub args: OverrideExpiryArgs,
}

pub struct OverrideExpiryInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub zeta_group: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for OverrideExpiry {
    type ArrangedAccounts = OverrideExpiryInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, zeta_group, _remaining @ ..] = accounts else {
            return None;
        };

        Some(OverrideExpiryInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
            zeta_group: zeta_group.pubkey,
        })
    }
}
