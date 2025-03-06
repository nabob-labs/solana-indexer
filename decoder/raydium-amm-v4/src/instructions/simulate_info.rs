
use solana_indexer_core::{borsh, IndexerDeserialize};
use super::super::types::*;


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0x0c")]
pub struct SimulateInfo{
    pub param: u8,
    pub swap_base_in_value: Option<SwapInstructionBaseIn>,
    pub swap_base_out_value: Option<SwapInstructionBaseOut>,
}

pub struct SimulateInfoInstructionAccounts {
    pub amm: solana_sdk::pubkey::Pubkey,
    pub amm_authority: solana_sdk::pubkey::Pubkey,
    pub amm_open_orders: solana_sdk::pubkey::Pubkey,
    pub pool_coin_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_pc_token_account: solana_sdk::pubkey::Pubkey,
    pub lp_mint_address: solana_sdk::pubkey::Pubkey,
    pub serum_market: solana_sdk::pubkey::Pubkey,
    pub serum_event_queue: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SimulateInfo {
    type ArrangedAccounts = SimulateInfoInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let amm = accounts.get(0)?;
        let amm_authority = accounts.get(1)?;
        let amm_open_orders = accounts.get(2)?;
        let pool_coin_token_account = accounts.get(3)?;
        let pool_pc_token_account = accounts.get(4)?;
        let lp_mint_address = accounts.get(5)?;
        let serum_market = accounts.get(6)?;
        let serum_event_queue = accounts.get(7)?;

        Some(SimulateInfoInstructionAccounts {
            amm: amm.pubkey,
            amm_authority: amm_authority.pubkey,
            amm_open_orders: amm_open_orders.pubkey,
            pool_coin_token_account: pool_coin_token_account.pubkey,
            pool_pc_token_account: pool_pc_token_account.pubkey,
            lp_mint_address: lp_mint_address.pubkey,
            serum_market: serum_market.pubkey,
            serum_event_queue: serum_event_queue.pubkey,
        })
    }
}
