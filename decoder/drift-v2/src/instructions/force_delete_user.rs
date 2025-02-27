use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x02f1c3ace318fe9e")]
pub struct ForceDeleteUser {}

pub struct ForceDeleteUserInstructionAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub user_stats: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub drift_signer: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ForceDeleteUser {
    type ArrangedAccounts = ForceDeleteUserInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, user_stats, state, authority, keeper, drift_signer, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(ForceDeleteUserInstructionAccounts {
            user: user.pubkey,
            user_stats: user_stats.pubkey,
            state: state.pubkey,
            authority: authority.pubkey,
            keeper: keeper.pubkey,
            drift_signer: drift_signer.pubkey,
        })
    }
}
