use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xd316ddfb4a79c12f")]
pub struct Pause {}

pub struct PauseInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub pause_authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Pause {
    type ArrangedAccounts = PauseInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, pause_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(PauseInstructionAccounts {
            state: state.pubkey,
            pause_authority: pause_authority.pubkey,
        })
    }
}
