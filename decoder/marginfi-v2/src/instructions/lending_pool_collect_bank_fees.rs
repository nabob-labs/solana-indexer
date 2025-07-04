use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xc905d774e65c4b96")]
pub struct LendingPoolCollectBankFees {}

pub struct LendingPoolCollectBankFeesInstructionAccounts {
    pub marginfi_group: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub liquidity_vault_authority: solana_pubkey::Pubkey,
    pub liquidity_vault: solana_pubkey::Pubkey,
    pub insurance_vault: solana_pubkey::Pubkey,
    pub fee_vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for LendingPoolCollectBankFees {
    type ArrangedAccounts = LendingPoolCollectBankFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_group, bank, liquidity_vault_authority, liquidity_vault, insurance_vault, fee_vault, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LendingPoolCollectBankFeesInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            bank: bank.pubkey,
            liquidity_vault_authority: liquidity_vault_authority.pubkey,
            liquidity_vault: liquidity_vault.pubkey,
            insurance_vault: insurance_vault.pubkey,
            fee_vault: fee_vault.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
