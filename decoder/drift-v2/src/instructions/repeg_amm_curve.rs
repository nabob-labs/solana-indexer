use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x03246659b48078d5")]
pub struct RepegAmmCurve {
    pub new_peg_candidate: u128,
}

pub struct RepegAmmCurveInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub perp_market: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for RepegAmmCurve {
    type ArrangedAccounts = RepegAmmCurveInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, perp_market, oracle, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(RepegAmmCurveInstructionAccounts {
            state: state.pubkey,
            perp_market: perp_market.pubkey,
            oracle: oracle.pubkey,
            admin: admin.pubkey,
        })
    }
}
