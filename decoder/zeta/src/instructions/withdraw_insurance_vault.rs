use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x11fad52dac7551e1")]
pub struct WithdrawInsuranceVault {
    pub percentage_amount: u64,
}

pub struct WithdrawInsuranceVaultInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub insurance_vault: solana_pubkey::Pubkey,
    pub insurance_deposit_account: solana_pubkey::Pubkey,
    pub user_token_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for WithdrawInsuranceVault {
    type ArrangedAccounts = WithdrawInsuranceVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, insurance_vault, insurance_deposit_account, user_token_account, authority, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawInsuranceVaultInstructionAccounts {
            state: state.pubkey,
            insurance_vault: insurance_vault.pubkey,
            insurance_deposit_account: insurance_deposit_account.pubkey,
            user_token_account: user_token_account.pubkey,
            authority: authority.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
