use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x679867665990c147")]
pub struct UpdatePerpMarketPerLpBase {
    pub per_lp_base: i8,
}

pub struct UpdatePerpMarketPerLpBaseInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
    pub perp_market: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdatePerpMarketPerLpBase {
    type ArrangedAccounts = UpdatePerpMarketPerLpBaseInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, perp_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdatePerpMarketPerLpBaseInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            perp_market: perp_market.pubkey,
        })
    }
}
