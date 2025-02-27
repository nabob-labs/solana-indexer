use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xc25cccdff6bc1fcb")]
pub struct UpdateUserMarginTradingEnabled {
    pub sub_account_id: u16,
    pub margin_trading_enabled: bool,
}

pub struct UpdateUserMarginTradingEnabledInstructionAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateUserMarginTradingEnabled {
    type ArrangedAccounts = UpdateUserMarginTradingEnabledInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateUserMarginTradingEnabledInstructionAccounts {
            user: user.pubkey,
            authority: authority.pubkey,
        })
    }
}
