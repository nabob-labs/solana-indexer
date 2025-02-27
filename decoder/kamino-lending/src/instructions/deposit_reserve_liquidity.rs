use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xa9c91e7e06cd6644")]
pub struct DepositReserveLiquidity {
    pub liquidity_amount: u64,
}

pub struct DepositReserveLiquidityInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub lending_market_authority: solana_sdk::pubkey::Pubkey,
    pub reserve_liquidity_mint: solana_sdk::pubkey::Pubkey,
    pub reserve_liquidity_supply: solana_sdk::pubkey::Pubkey,
    pub reserve_collateral_mint: solana_sdk::pubkey::Pubkey,
    pub user_source_liquidity: solana_sdk::pubkey::Pubkey,
    pub user_destination_collateral: solana_sdk::pubkey::Pubkey,
    pub collateral_token_program: solana_sdk::pubkey::Pubkey,
    pub liquidity_token_program: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for DepositReserveLiquidity {
    type ArrangedAccounts = DepositReserveLiquidityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, reserve, lending_market, lending_market_authority, reserve_liquidity_mint, reserve_liquidity_supply, reserve_collateral_mint, user_source_liquidity, user_destination_collateral, collateral_token_program, liquidity_token_program, instruction_sysvar_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositReserveLiquidityInstructionAccounts {
            owner: owner.pubkey,
            reserve: reserve.pubkey,
            lending_market: lending_market.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            reserve_liquidity_mint: reserve_liquidity_mint.pubkey,
            reserve_liquidity_supply: reserve_liquidity_supply.pubkey,
            reserve_collateral_mint: reserve_collateral_mint.pubkey,
            user_source_liquidity: user_source_liquidity.pubkey,
            user_destination_collateral: user_destination_collateral.pubkey,
            collateral_token_program: collateral_token_program.pubkey,
            liquidity_token_program: liquidity_token_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}
