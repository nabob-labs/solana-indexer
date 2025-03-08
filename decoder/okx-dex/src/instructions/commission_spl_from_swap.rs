use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x054d9032dee4e9ab")]
pub struct CommissionSplFromSwap {
    pub args: SwapArgs,
    pub commission_rate: u16,
    pub bridge_to_args: BridgeToArgs,
    pub offset: u8,
    pub len: u8,
}

pub struct CommissionSplFromSwapInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub source_token_account: solana_sdk::pubkey::Pubkey,
    pub destination_token_account: solana_sdk::pubkey::Pubkey,
    pub source_mint: solana_sdk::pubkey::Pubkey,
    pub destination_mint: solana_sdk::pubkey::Pubkey,
    pub bridge_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_2022_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub commission_token_account: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CommissionSplFromSwap {
    type ArrangedAccounts = CommissionSplFromSwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, source_token_account, destination_token_account, source_mint, destination_mint, bridge_program, associated_token_program, token_program, token_2022_program, system_program, commission_token_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CommissionSplFromSwapInstructionAccounts {
            payer: payer.pubkey,
            source_token_account: source_token_account.pubkey,
            destination_token_account: destination_token_account.pubkey,
            source_mint: source_mint.pubkey,
            destination_mint: destination_mint.pubkey,
            bridge_program: bridge_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            token_program: token_program.pubkey,
            token_2022_program: token_2022_program.pubkey,
            system_program: system_program.pubkey,
            commission_token_account: commission_token_account.pubkey,
        })
    }
}
