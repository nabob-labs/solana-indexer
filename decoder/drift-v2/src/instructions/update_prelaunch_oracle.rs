use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xdc841b1be9dc3ddb")]
pub struct UpdatePrelaunchOracle {}

pub struct UpdatePrelaunchOracleInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub perp_market: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdatePrelaunchOracle {
    type ArrangedAccounts = UpdatePrelaunchOracleInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, perp_market, oracle, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdatePrelaunchOracleInstructionAccounts {
            state: state.pubkey,
            perp_market: perp_market.pubkey,
            oracle: oracle.pubkey,
        })
    }
}
