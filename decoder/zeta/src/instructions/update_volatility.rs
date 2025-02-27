use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xbe6974dde5c6d053")]
pub struct UpdateVolatility {
    pub args: UpdateVolatilityArgs,
}

pub struct UpdateVolatilityInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub greeks: solana_sdk::pubkey::Pubkey,
    pub zeta_group: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateVolatility {
    type ArrangedAccounts = UpdateVolatilityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, greeks, zeta_group, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateVolatilityInstructionAccounts {
            state: state.pubkey,
            greeks: greeks.pubkey,
            zeta_group: zeta_group.pubkey,
            admin: admin.pubkey,
        })
    }
}
