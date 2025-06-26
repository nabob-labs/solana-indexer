use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x3ba964314511adfd")]
pub struct DeletePrelaunchOracle {
    pub perp_market_index: u16,
}

pub struct DeletePrelaunchOracleInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub prelaunch_oracle: solana_pubkey::Pubkey,
    pub perp_market: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for DeletePrelaunchOracle {
    type ArrangedAccounts = DeletePrelaunchOracleInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, prelaunch_oracle, perp_market, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(DeletePrelaunchOracleInstructionAccounts {
            admin: admin.pubkey,
            prelaunch_oracle: prelaunch_oracle.pubkey,
            perp_market: perp_market.pubkey,
            state: state.pubkey,
        })
    }
}
