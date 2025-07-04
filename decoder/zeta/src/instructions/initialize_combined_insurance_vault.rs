use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x4d12b590db54066a")]
pub struct InitializeCombinedInsuranceVault {
    pub nonce: u8,
}

pub struct InitializeCombinedInsuranceVaultInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub insurance_vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub usdc_mint: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeCombinedInsuranceVault {
    type ArrangedAccounts = InitializeCombinedInsuranceVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, insurance_vault, token_program, usdc_mint, admin, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeCombinedInsuranceVaultInstructionAccounts {
            state: state.pubkey,
            insurance_vault: insurance_vault.pubkey,
            token_program: token_program.pubkey,
            usdc_mint: usdc_mint.pubkey,
            admin: admin.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
