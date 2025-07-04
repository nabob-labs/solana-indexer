use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x44c8e4e9b820e2bc")]
pub struct HarvestReward {
    pub reward_index: u64,
}

pub struct HarvestRewardInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub user_state: solana_pubkey::Pubkey,
    pub farm_state: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub reward_mint: solana_pubkey::Pubkey,
    pub user_reward_ata: solana_pubkey::Pubkey,
    pub rewards_vault: solana_pubkey::Pubkey,
    pub rewards_treasury_vault: solana_pubkey::Pubkey,
    pub farm_vaults_authority: solana_pubkey::Pubkey,
    pub scope_prices: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for HarvestReward {
    type ArrangedAccounts = HarvestRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, user_state, farm_state, global_config, reward_mint, user_reward_ata, rewards_vault, rewards_treasury_vault, farm_vaults_authority, scope_prices, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(HarvestRewardInstructionAccounts {
            owner: owner.pubkey,
            user_state: user_state.pubkey,
            farm_state: farm_state.pubkey,
            global_config: global_config.pubkey,
            reward_mint: reward_mint.pubkey,
            user_reward_ata: user_reward_ata.pubkey,
            rewards_vault: rewards_vault.pubkey,
            rewards_treasury_vault: rewards_treasury_vault.pubkey,
            farm_vaults_authority: farm_vaults_authority.pubkey,
            scope_prices: scope_prices.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
