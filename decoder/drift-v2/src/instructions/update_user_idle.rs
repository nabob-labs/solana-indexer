use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xfd85431667a11464")]
pub struct UpdateUserIdle {}

pub struct UpdateUserIdleInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub filler: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateUserIdle {
    type ArrangedAccounts = UpdateUserIdleInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, authority, filler, user, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateUserIdleInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
            filler: filler.pubkey,
            user: user.pubkey,
        })
    }
}
