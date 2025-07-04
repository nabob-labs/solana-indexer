use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x4419332b7eab5057")]
pub struct InitializeMinLotsAndTickSizes {}

pub struct InitializeMinLotsAndTickSizesInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeMinLotsAndTickSizes {
    type ArrangedAccounts = InitializeMinLotsAndTickSizesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeMinLotsAndTickSizesInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
        })
    }
}
