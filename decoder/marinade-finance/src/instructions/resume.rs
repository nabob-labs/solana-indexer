use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x01a633aa7f208dce")]
pub struct Resume {}

pub struct ResumeInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub pause_authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Resume {
    type ArrangedAccounts = ResumeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, pause_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ResumeInstructionAccounts {
            state: state.pubkey,
            pause_authority: pause_authority.pubkey,
        })
    }
}
