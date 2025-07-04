use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x8784cda56d96a66a")]
pub struct UpdatePerpMarketUnrealizedAssetWeight {
    pub unrealized_initial_asset_weight: u32,
    pub unrealized_maintenance_asset_weight: u32,
}

pub struct UpdatePerpMarketUnrealizedAssetWeightInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub perp_market: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdatePerpMarketUnrealizedAssetWeight {
    type ArrangedAccounts = UpdatePerpMarketUnrealizedAssetWeightInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, perp_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdatePerpMarketUnrealizedAssetWeightInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            perp_market: perp_market.pubkey,
        })
    }
}
