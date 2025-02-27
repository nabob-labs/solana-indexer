use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x3e5744731d9696a5")]
pub struct UpdatePerpMarketTargetBaseAssetAmountPerLp {
    pub target_base_asset_amount_per_lp: i32,
}

pub struct UpdatePerpMarketTargetBaseAssetAmountPerLpInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
    pub perp_market: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdatePerpMarketTargetBaseAssetAmountPerLp {
    type ArrangedAccounts = UpdatePerpMarketTargetBaseAssetAmountPerLpInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, perp_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(
            UpdatePerpMarketTargetBaseAssetAmountPerLpInstructionAccounts {
                admin: admin.pubkey,
                state: state.pubkey,
                perp_market: perp_market.pubkey,
            },
        )
    }
}
