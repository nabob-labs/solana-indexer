use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xc77ac0ff20633fc8")]
pub struct ResetFuelSeason {}

pub struct ResetFuelSeasonInstructionAccounts {
    pub user_stats: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ResetFuelSeason {
    type ArrangedAccounts = ResetFuelSeasonInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user_stats, authority, state, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ResetFuelSeasonInstructionAccounts {
            user_stats: user_stats.pubkey,
            authority: authority.pubkey,
            state: state.pubkey,
            admin: admin.pubkey,
        })
    }
}
