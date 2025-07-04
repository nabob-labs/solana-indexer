use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x12eda6c52210d590")]
pub struct CollectRemainingRewards {
    pub reward_index: u8,
}

#[derive(Debug, PartialEq)]
pub struct CollectRemainingRewardsInstructionAccounts {
    pub reward_funder: solana_pubkey::Pubkey,
    pub funder_token_account: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub reward_token_vault: solana_pubkey::Pubkey,
    pub reward_vault_mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_program2022: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CollectRemainingRewards {
    type ArrangedAccounts = CollectRemainingRewardsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [reward_funder, funder_token_account, pool_state, reward_token_vault, reward_vault_mint, token_program, token_program2022, memo_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CollectRemainingRewardsInstructionAccounts {
            reward_funder: reward_funder.pubkey,
            funder_token_account: funder_token_account.pubkey,
            pool_state: pool_state.pubkey,
            reward_token_vault: reward_token_vault.pubkey,
            reward_vault_mint: reward_vault_mint.pubkey,
            token_program: token_program.pubkey,
            token_program2022: token_program2022.pubkey,
            memo_program: memo_program.pubkey,
        })
    }
}
