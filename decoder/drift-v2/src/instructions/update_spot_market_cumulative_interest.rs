use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x27a68bf39ea59be1")]
pub struct UpdateSpotMarketCumulativeInterest {}

pub struct UpdateSpotMarketCumulativeInterestInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub spot_market: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub spot_market_vault: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateSpotMarketCumulativeInterest {
    type ArrangedAccounts = UpdateSpotMarketCumulativeInterestInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, spot_market, oracle, spot_market_vault, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateSpotMarketCumulativeInterestInstructionAccounts {
            state: state.pubkey,
            spot_market: spot_market.pubkey,
            oracle: oracle.pubkey,
            spot_market_vault: spot_market_vault.pubkey,
        })
    }
}
