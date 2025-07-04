use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xf14a727bce9918ca")]
pub struct UpdatePerpMarketAmmOracleTwap {}

pub struct UpdatePerpMarketAmmOracleTwapInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub perp_market: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdatePerpMarketAmmOracleTwap {
    type ArrangedAccounts = UpdatePerpMarketAmmOracleTwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, perp_market, oracle, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdatePerpMarketAmmOracleTwapInstructionAccounts {
            state: state.pubkey,
            perp_market: perp_market.pubkey,
            oracle: oracle.pubkey,
            admin: admin.pubkey,
        })
    }
}
