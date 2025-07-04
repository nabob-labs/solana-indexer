use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xa21523fb2039a1d2")]
pub struct LogUserBalances {}

pub struct LogUserBalancesInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for LogUserBalances {
    type ArrangedAccounts = LogUserBalancesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, authority, user, _remaining @ ..] = accounts else {
            return None;
        };

        Some(LogUserBalancesInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
            user: user.pubkey,
        })
    }
}
