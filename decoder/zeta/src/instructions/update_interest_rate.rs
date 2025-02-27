use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x4b08ff297b3b87ee")]
pub struct UpdateInterestRate {
    pub args: UpdateInterestRateArgs,
}

pub struct UpdateInterestRateInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub greeks: solana_sdk::pubkey::Pubkey,
    pub zeta_group: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateInterestRate {
    type ArrangedAccounts = UpdateInterestRateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, greeks, zeta_group, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateInterestRateInstructionAccounts {
            state: state.pubkey,
            greeks: greeks.pubkey,
            zeta_group: zeta_group.pubkey,
            admin: admin.pubkey,
        })
    }
}
