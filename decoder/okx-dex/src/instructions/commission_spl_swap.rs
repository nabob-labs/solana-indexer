use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xeb47d3c472c78f5c")]
pub struct CommissionSplSwap {
    pub data: CommissionSwapArgs,
}

pub struct CommissionSplSwapInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub source_token_account: solana_pubkey::Pubkey,
    pub destination_token_account: solana_pubkey::Pubkey,
    pub source_mint: solana_pubkey::Pubkey,
    pub destination_mint: solana_pubkey::Pubkey,
    pub commission_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CommissionSplSwap {
    type ArrangedAccounts = CommissionSplSwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, source_token_account, destination_token_account, source_mint, destination_mint, commission_token_account, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CommissionSplSwapInstructionAccounts {
            payer: payer.pubkey,
            source_token_account: source_token_account.pubkey,
            destination_token_account: destination_token_account.pubkey,
            source_mint: source_mint.pubkey,
            destination_mint: destination_mint.pubkey,
            commission_token_account: commission_token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
