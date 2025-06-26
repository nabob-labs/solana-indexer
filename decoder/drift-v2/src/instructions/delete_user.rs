use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xba5511f9dbe762fb")]
pub struct DeleteUser {}

pub struct DeleteUserInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for DeleteUser {
    type ArrangedAccounts = DeleteUserInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, user_stats, state, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(DeleteUserInstructionAccounts {
            user: user.pubkey,
            user_stats: user_stats.pubkey,
            state: state.pubkey,
            authority: authority.pubkey,
        })
    }
}
