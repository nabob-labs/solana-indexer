use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xbfbbb0890919bbf4")]
pub struct WithdrawReward {
    pub amount: u64,
    pub reward_index: u64,
}

pub struct WithdrawRewardInstructionAccounts {
    pub farm_admin: solana_pubkey::Pubkey,
    pub farm_state: solana_pubkey::Pubkey,
    pub reward_mint: solana_pubkey::Pubkey,
    pub reward_vault: solana_pubkey::Pubkey,
    pub farm_vaults_authority: solana_pubkey::Pubkey,
    pub admin_reward_token_ata: solana_pubkey::Pubkey,
    pub scope_prices: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for WithdrawReward {
    type ArrangedAccounts = WithdrawRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [farm_admin, farm_state, reward_mint, reward_vault, farm_vaults_authority, admin_reward_token_ata, scope_prices, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawRewardInstructionAccounts {
            farm_admin: farm_admin.pubkey,
            farm_state: farm_state.pubkey,
            reward_mint: reward_mint.pubkey,
            reward_vault: reward_vault.pubkey,
            farm_vaults_authority: farm_vaults_authority.pubkey,
            admin_reward_token_ata: admin_reward_token_ata.pubkey,
            scope_prices: scope_prices.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
