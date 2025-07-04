use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x4cd3d5ab754e9e4c")]
pub struct LendingPoolAddBankWithSeed {
    pub bank_config: BankConfigCompact,
    pub bank_seed: u64,
}

pub struct LendingPoolAddBankWithSeedInstructionAccounts {
    pub marginfi_group: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub fee_payer: solana_pubkey::Pubkey,
    pub bank_mint: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub liquidity_vault_authority: solana_pubkey::Pubkey,
    pub liquidity_vault: solana_pubkey::Pubkey,
    pub insurance_vault_authority: solana_pubkey::Pubkey,
    pub insurance_vault: solana_pubkey::Pubkey,
    pub fee_vault_authority: solana_pubkey::Pubkey,
    pub fee_vault: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for LendingPoolAddBankWithSeed {
    type ArrangedAccounts = LendingPoolAddBankWithSeedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_group, admin, fee_payer, bank_mint, bank, liquidity_vault_authority, liquidity_vault, insurance_vault_authority, insurance_vault, fee_vault_authority, fee_vault, rent, token_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LendingPoolAddBankWithSeedInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            admin: admin.pubkey,
            fee_payer: fee_payer.pubkey,
            bank_mint: bank_mint.pubkey,
            bank: bank.pubkey,
            liquidity_vault_authority: liquidity_vault_authority.pubkey,
            liquidity_vault: liquidity_vault.pubkey,
            insurance_vault_authority: insurance_vault_authority.pubkey,
            insurance_vault: insurance_vault.pubkey,
            fee_vault_authority: fee_vault_authority.pubkey,
            fee_vault: fee_vault.pubkey,
            rent: rent.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
