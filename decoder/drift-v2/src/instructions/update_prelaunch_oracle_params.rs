use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x62cd93f3124b53cf")]
pub struct UpdatePrelaunchOracleParams {
    pub params: PrelaunchOracleParams,
}

pub struct UpdatePrelaunchOracleParamsInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub prelaunch_oracle: solana_pubkey::Pubkey,
    pub perp_market: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdatePrelaunchOracleParams {
    type ArrangedAccounts = UpdatePrelaunchOracleParamsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, prelaunch_oracle, perp_market, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdatePrelaunchOracleParamsInstructionAccounts {
            admin: admin.pubkey,
            prelaunch_oracle: prelaunch_oracle.pubkey,
            perp_market: perp_market.pubkey,
            state: state.pubkey,
        })
    }
}
