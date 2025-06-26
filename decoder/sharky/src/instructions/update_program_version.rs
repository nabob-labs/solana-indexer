use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xeb84d7e1d52b2b26")]
pub struct UpdateProgramVersion {
    pub version: u8,
}

pub struct UpdateProgramVersionInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub program_version: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateProgramVersion {
    type ArrangedAccounts = UpdateProgramVersionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, program_version, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateProgramVersionInstructionAccounts {
            authority: authority.pubkey,
            program_version: program_version.pubkey,
        })
    }
}
