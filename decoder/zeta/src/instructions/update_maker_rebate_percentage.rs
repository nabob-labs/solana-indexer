use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xb4ecfd13e7e7dc41")]
pub struct UpdateMakerRebatePercentage {
    pub native_maker_rebate_percentage: u64,
}

pub struct UpdateMakerRebatePercentageInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateMakerRebatePercentage {
    type ArrangedAccounts = UpdateMakerRebatePercentageInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateMakerRebatePercentageInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
        })
    }
}
