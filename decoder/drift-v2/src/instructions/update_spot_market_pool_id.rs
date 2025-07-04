use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x16d5c5a08bc15195")]
pub struct UpdateSpotMarketPoolId {
    pub pool_id: u8,
}

pub struct UpdateSpotMarketPoolIdInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub spot_market: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateSpotMarketPoolId {
    type ArrangedAccounts = UpdateSpotMarketPoolIdInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, spot_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateSpotMarketPoolIdInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            spot_market: spot_market.pubkey,
        })
    }
}
