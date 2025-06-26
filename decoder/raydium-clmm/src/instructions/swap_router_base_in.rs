use alloc::vec::Vec;
use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x457d73daf5baf2c4")]
pub struct SwapRouterBaseIn {
    pub amount_in: u64,
    pub amount_out_minimum: u64,
}

#[derive(Debug, PartialEq)]
pub struct SwapRouterBaseInInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub input_token_account: solana_pubkey::Pubkey,
    pub input_token_mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_program2022: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SwapRouterBaseIn {
    type ArrangedAccounts = SwapRouterBaseInInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, input_token_account, input_token_mint, token_program, token_program2022, memo_program, remaining_accounts @ ..] =
            accounts
        else {
            return None;
        };

        Some(SwapRouterBaseInInstructionAccounts {
            payer: payer.pubkey,
            input_token_account: input_token_account.pubkey,
            input_token_mint: input_token_mint.pubkey,
            token_program: token_program.pubkey,
            token_program2022: token_program2022.pubkey,
            memo_program: memo_program.pubkey,
            remaining_accounts: remaining_accounts.to_vec(),
        })
    }
}
