use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x7a65f9eed109f1f5")]
pub struct UpdatePerpMarketAmmSummaryStats {
    pub params: UpdatePerpMarketSummaryStatsParams,
}

pub struct UpdatePerpMarketAmmSummaryStatsInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub perp_market: solana_pubkey::Pubkey,
    pub spot_market: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdatePerpMarketAmmSummaryStats {
    type ArrangedAccounts = UpdatePerpMarketAmmSummaryStatsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, perp_market, spot_market, oracle, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdatePerpMarketAmmSummaryStatsInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            perp_market: perp_market.pubkey,
            spot_market: spot_market.pubkey,
            oracle: oracle.pubkey,
        })
    }
}
