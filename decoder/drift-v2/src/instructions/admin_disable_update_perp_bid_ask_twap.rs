use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x11a4522db756bfc7")]
pub struct AdminDisableUpdatePerpBidAskTwap {
    pub disable: bool,
}

pub struct AdminDisableUpdatePerpBidAskTwapInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for AdminDisableUpdatePerpBidAskTwap {
    type ArrangedAccounts = AdminDisableUpdatePerpBidAskTwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, user_stats, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AdminDisableUpdatePerpBidAskTwapInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            user_stats: user_stats.pubkey,
        })
    }
}
