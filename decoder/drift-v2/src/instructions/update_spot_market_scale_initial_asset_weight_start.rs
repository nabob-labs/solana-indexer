use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xd9cccc76cc82e193")]
pub struct UpdateSpotMarketScaleInitialAssetWeightStart {
    pub scale_initial_asset_weight_start: u64,
}

pub struct UpdateSpotMarketScaleInitialAssetWeightStartInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub spot_market: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateSpotMarketScaleInitialAssetWeightStart {
    type ArrangedAccounts = UpdateSpotMarketScaleInitialAssetWeightStartInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, spot_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(
            UpdateSpotMarketScaleInitialAssetWeightStartInstructionAccounts {
                admin: admin.pubkey,
                state: state.pubkey,
                spot_market: spot_market.pubkey,
            },
        )
    }
}
