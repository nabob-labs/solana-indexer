use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0c2bb0539cfb750d")]
pub struct LiquidateSpotWithSwapBegin {
    pub asset_market_index: u16,
    pub liability_market_index: u16,
    pub swap_amount: u64,
}

pub struct LiquidateSpotWithSwapBeginInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub liquidator: solana_pubkey::Pubkey,
    pub liquidator_stats: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub liability_spot_market_vault: solana_pubkey::Pubkey,
    pub asset_spot_market_vault: solana_pubkey::Pubkey,
    pub liability_token_account: solana_pubkey::Pubkey,
    pub asset_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub drift_signer: solana_pubkey::Pubkey,
    pub instructions: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for LiquidateSpotWithSwapBegin {
    type ArrangedAccounts = LiquidateSpotWithSwapBeginInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, authority, liquidator, liquidator_stats, user, user_stats, liability_spot_market_vault, asset_spot_market_vault, liability_token_account, asset_token_account, token_program, drift_signer, instructions, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LiquidateSpotWithSwapBeginInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
            liquidator: liquidator.pubkey,
            liquidator_stats: liquidator_stats.pubkey,
            user: user.pubkey,
            user_stats: user_stats.pubkey,
            liability_spot_market_vault: liability_spot_market_vault.pubkey,
            asset_spot_market_vault: asset_spot_market_vault.pubkey,
            liability_token_account: liability_token_account.pubkey,
            asset_token_account: asset_token_account.pubkey,
            token_program: token_program.pubkey,
            drift_signer: drift_signer.pubkey,
            instructions: instructions.pubkey,
        })
    }
}
