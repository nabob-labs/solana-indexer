use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xc1209b3341d69c81")]
pub struct SharedAccountsRoute {
    pub id: u8,
    pub route_plan: Vec<RoutePlanStep>,
    pub in_amount: u64,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}

pub struct SharedAccountsRouteInstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub program_authority: solana_pubkey::Pubkey,
    pub user_transfer_authority: solana_pubkey::Pubkey,
    pub source_token_account: solana_pubkey::Pubkey,
    pub program_source_token_account: solana_pubkey::Pubkey,
    pub program_destination_token_account: solana_pubkey::Pubkey,
    pub destination_token_account: solana_pubkey::Pubkey,
    pub source_mint: solana_pubkey::Pubkey,
    pub destination_mint: solana_pubkey::Pubkey,
    pub platform_fee_account: solana_pubkey::Pubkey,
    pub token_2022_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SharedAccountsRoute {
    type ArrangedAccounts = SharedAccountsRouteInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_program, program_authority, user_transfer_authority, source_token_account, program_source_token_account, program_destination_token_account, destination_token_account, source_mint, destination_mint, platform_fee_account, token_2022_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SharedAccountsRouteInstructionAccounts {
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
            token_2022_program: token_2022_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
