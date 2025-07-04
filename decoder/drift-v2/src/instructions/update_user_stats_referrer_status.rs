use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xae9a482abf9491cd")]
pub struct UpdateUserStatsReferrerStatus {}

pub struct UpdateUserStatsReferrerStatusInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateUserStatsReferrerStatus {
    type ArrangedAccounts = UpdateUserStatsReferrerStatusInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, authority, user_stats, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateUserStatsReferrerStatusInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
            user_stats: user_stats.pubkey,
        })
    }
}
