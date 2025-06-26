use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x851d59df45eeb00a")]
pub struct IncreaseLiquidityV2 {
    pub liquidity_amount: u128,
    pub token_max_a: u64,
    pub token_max_b: u64,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

#[derive(Debug, PartialEq)]
pub struct IncreaseLiquidityV2InstructionAccounts {
    pub whirlpool: solana_pubkey::Pubkey,
    pub token_program_a: solana_pubkey::Pubkey,
    pub token_program_b: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
    pub position_authority: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_token_account: solana_pubkey::Pubkey,
    pub token_mint_a: solana_pubkey::Pubkey,
    pub token_mint_b: solana_pubkey::Pubkey,
    pub token_owner_account_a: solana_pubkey::Pubkey,
    pub token_owner_account_b: solana_pubkey::Pubkey,
    pub token_vault_a: solana_pubkey::Pubkey,
    pub token_vault_b: solana_pubkey::Pubkey,
    pub tick_array_lower: solana_pubkey::Pubkey,
    pub tick_array_upper: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for IncreaseLiquidityV2 {
    type ArrangedAccounts = IncreaseLiquidityV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whirlpool, token_program_a, token_program_b, memo_program, position_authority, position, position_token_account, token_mint_a, token_mint_b, token_owner_account_a, token_owner_account_b, token_vault_a, token_vault_b, tick_array_lower, tick_array_upper, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(IncreaseLiquidityV2InstructionAccounts {
            whirlpool: whirlpool.pubkey,
            token_program_a: token_program_a.pubkey,
            token_program_b: token_program_b.pubkey,
            memo_program: memo_program.pubkey,
            position_authority: position_authority.pubkey,
            position: position.pubkey,
            position_token_account: position_token_account.pubkey,
            token_mint_a: token_mint_a.pubkey,
            token_mint_b: token_mint_b.pubkey,
            token_owner_account_a: token_owner_account_a.pubkey,
            token_owner_account_b: token_owner_account_b.pubkey,
            token_vault_a: token_vault_a.pubkey,
            token_vault_b: token_vault_b.pubkey,
            tick_array_lower: tick_array_lower.pubkey,
            tick_array_upper: tick_array_upper.pubkey,
        })
    }
}
