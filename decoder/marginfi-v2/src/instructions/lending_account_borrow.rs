use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x047e74353005d41f")]
pub struct LendingAccountBorrow {
    pub amount: u64,
}

pub struct LendingAccountBorrowInstructionAccounts {
    pub marginfi_group: solana_pubkey::Pubkey,
    pub marginfi_account: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub destination_token_account: solana_pubkey::Pubkey,
    pub bank_liquidity_vault_authority: solana_pubkey::Pubkey,
    pub bank_liquidity_vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for LendingAccountBorrow {
    type ArrangedAccounts = LendingAccountBorrowInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_group, marginfi_account, signer, bank, destination_token_account, bank_liquidity_vault_authority, bank_liquidity_vault, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LendingAccountBorrowInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            marginfi_account: marginfi_account.pubkey,
            signer: signer.pubkey,
            bank: bank.pubkey,
            destination_token_account: destination_token_account.pubkey,
            bank_liquidity_vault_authority: bank_liquidity_vault_authority.pubkey,
            bank_liquidity_vault: bank_liquidity_vault.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
