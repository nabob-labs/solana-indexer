use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x5b9a18576a3bbe42")]
pub struct DeleteInitializedPerpMarket {
    pub market_index: u16,
}

pub struct DeleteInitializedPerpMarketInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub perp_market: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for DeleteInitializedPerpMarket {
    type ArrangedAccounts = DeleteInitializedPerpMarketInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, perp_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(DeleteInitializedPerpMarketInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            perp_market: perp_market.pubkey,
        })
    }
}
