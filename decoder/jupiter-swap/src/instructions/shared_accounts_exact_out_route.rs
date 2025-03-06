
use solana_indexer_core::{borsh, IndexerDeserialize};
use super::super::types::*;


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0xb0d169a89a7d453e")]
pub struct SharedAccountsExactOutRoute{
    pub id: u8,
    pub route_plan: Vec<RoutePlanStep>,
    pub out_amount: u64,
    pub quoted_in_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}

pub struct SharedAccountsExactOutRouteInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub program_authority: solana_sdk::pubkey::Pubkey,
    pub user_transfer_authority: solana_sdk::pubkey::Pubkey,
    pub source_token_account: solana_sdk::pubkey::Pubkey,
    pub program_source_token_account: solana_sdk::pubkey::Pubkey,
    pub program_destination_token_account: solana_sdk::pubkey::Pubkey,
    pub destination_token_account: solana_sdk::pubkey::Pubkey,
    pub source_mint: solana_sdk::pubkey::Pubkey,
    pub destination_mint: solana_sdk::pubkey::Pubkey,
    pub platform_fee_account: solana_sdk::pubkey::Pubkey,
    pub token2022_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SharedAccountsExactOutRoute {
    type ArrangedAccounts = SharedAccountsExactOutRouteInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let program_authority = accounts.get(1)?;
        let user_transfer_authority = accounts.get(2)?;
        let source_token_account = accounts.get(3)?;
        let program_source_token_account = accounts.get(4)?;
        let program_destination_token_account = accounts.get(5)?;
        let destination_token_account = accounts.get(6)?;
        let source_mint = accounts.get(7)?;
        let destination_mint = accounts.get(8)?;
        let platform_fee_account = accounts.get(9)?;
        let token2022_program = accounts.get(10)?;
        let event_authority = accounts.get(11)?;
        let program = accounts.get(12)?;

        Some(SharedAccountsExactOutRouteInstructionAccounts {
            token_program: token_program.pubkey,
            program_authority: program_authority.pubkey,
            user_transfer_authority: user_transfer_authority.pubkey,
            source_token_account: source_token_account.pubkey,
            program_source_token_account: program_source_token_account.pubkey,
            program_destination_token_account: program_destination_token_account.pubkey,
            destination_token_account: destination_token_account.pubkey,
            source_mint: source_mint.pubkey,
            destination_mint: destination_mint.pubkey,
            platform_fee_account: platform_fee_account.pubkey,
            token2022_program: token2022_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
