use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xd285e180c2320d6d")]
pub struct UpdateInitialPctToLiquidate {
    pub initial_pct_to_liquidate: u16,
}

pub struct UpdateInitialPctToLiquidateInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateInitialPctToLiquidate {
    type ArrangedAccounts = UpdateInitialPctToLiquidateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateInitialPctToLiquidateInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
