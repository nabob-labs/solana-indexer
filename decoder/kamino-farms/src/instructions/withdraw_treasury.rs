use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x283f7a9e90d85360")]
pub struct WithdrawTreasury {
    pub amount: u64,
}

pub struct WithdrawTreasuryInstructionAccounts {
    pub global_admin: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub reward_mint: solana_pubkey::Pubkey,
    pub reward_treasury_vault: solana_pubkey::Pubkey,
    pub treasury_vault_authority: solana_pubkey::Pubkey,
    pub withdraw_destination_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for WithdrawTreasury {
    type ArrangedAccounts = WithdrawTreasuryInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [global_admin, global_config, reward_mint, reward_treasury_vault, treasury_vault_authority, withdraw_destination_token_account, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawTreasuryInstructionAccounts {
            global_admin: global_admin.pubkey,
            global_config: global_config.pubkey,
            reward_mint: reward_mint.pubkey,
            reward_treasury_vault: reward_treasury_vault.pubkey,
            treasury_vault_authority: treasury_vault_authority.pubkey,
            withdraw_destination_token_account: withdraw_destination_token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
