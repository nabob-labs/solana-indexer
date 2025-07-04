use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xed4bc6ebe9ba4b23")]
pub struct LiquidatePerpPnlForDeposit {
    pub perp_market_index: u16,
    pub spot_market_index: u16,
    pub liquidator_max_pnl_transfer: u128,
    pub limit_price: Option<u64>,
}

pub struct LiquidatePerpPnlForDepositInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub liquidator: solana_pubkey::Pubkey,
    pub liquidator_stats: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for LiquidatePerpPnlForDeposit {
    type ArrangedAccounts = LiquidatePerpPnlForDepositInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, authority, liquidator, liquidator_stats, user, user_stats, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LiquidatePerpPnlForDepositInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
            liquidator: liquidator.pubkey,
            liquidator_stats: liquidator_stats.pubkey,
            user: user.pubkey,
            user_stats: user_stats.pubkey,
        })
    }
}
