use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x72b86625f6bab463")]
pub struct UpdateSpotMarketOracle {
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub oracle_source: OracleSource,
    pub skip_invariant_check: bool,
}

pub struct UpdateSpotMarketOracleInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
    pub spot_market: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub old_oracle: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateSpotMarketOracle {
    type ArrangedAccounts = UpdateSpotMarketOracleInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, spot_market, oracle, old_oracle, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateSpotMarketOracleInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            spot_market: spot_market.pubkey,
            oracle: oracle.pubkey,
            old_oracle: old_oracle.pubkey,
        })
    }
}
