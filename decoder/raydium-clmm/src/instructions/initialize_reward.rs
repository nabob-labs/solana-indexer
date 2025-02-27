use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x5f87c0c4f281e644")]
pub struct InitializeReward {
    pub param: InitializeRewardParam,
}

pub struct InitializeRewardInstructionAccounts {
    pub reward_funder: solana_sdk::pubkey::Pubkey,
    pub funder_token_account: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub operation_state: solana_sdk::pubkey::Pubkey,
    pub reward_token_mint: solana_sdk::pubkey::Pubkey,
    pub reward_token_vault: solana_sdk::pubkey::Pubkey,
    pub reward_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeReward {
    type ArrangedAccounts = InitializeRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [reward_funder, funder_token_account, amm_config, pool_state, operation_state, reward_token_mint, reward_token_vault, reward_token_program, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeRewardInstructionAccounts {
            reward_funder: reward_funder.pubkey,
            funder_token_account: funder_token_account.pubkey,
            amm_config: amm_config.pubkey,
            pool_state: pool_state.pubkey,
            operation_state: operation_state.pubkey,
            reward_token_mint: reward_token_mint.pubkey,
            reward_token_vault: reward_token_vault.pubkey,
            reward_token_program: reward_token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
