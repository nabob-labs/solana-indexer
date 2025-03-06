use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x02")]
pub struct MonitorStep {
    pub plan_order_limit: u16,
    pub place_order_limit: u16,
    pub cancel_order_limit: u16,
}

pub struct MonitorStepInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub amm: solana_sdk::pubkey::Pubkey,
    pub amm_authority: solana_sdk::pubkey::Pubkey,
    pub amm_open_orders: solana_sdk::pubkey::Pubkey,
    pub amm_target_orders: solana_sdk::pubkey::Pubkey,
    pub pool_coin_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_pc_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_withdraw_queue: solana_sdk::pubkey::Pubkey,
    pub serum_program: solana_sdk::pubkey::Pubkey,
    pub serum_market: solana_sdk::pubkey::Pubkey,
    pub serum_coin_vault_account: solana_sdk::pubkey::Pubkey,
    pub serum_pc_vault_account: solana_sdk::pubkey::Pubkey,
    pub serum_vault_signer: solana_sdk::pubkey::Pubkey,
    pub serum_req_q: solana_sdk::pubkey::Pubkey,
    pub serum_event_q: solana_sdk::pubkey::Pubkey,
    pub serum_bids: solana_sdk::pubkey::Pubkey,
    pub serum_asks: solana_sdk::pubkey::Pubkey,
    pub serum_fee_discount: solana_sdk::pubkey::Pubkey,
    pub referrer_pc_account: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for MonitorStep {
    type ArrangedAccounts = MonitorStepInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let rent = accounts.get(1)?;
        let clock = accounts.get(2)?;
        let amm = accounts.get(3)?;
        let amm_authority = accounts.get(4)?;
        let amm_open_orders = accounts.get(5)?;
        let amm_target_orders = accounts.get(6)?;
        let pool_coin_token_account = accounts.get(7)?;
        let pool_pc_token_account = accounts.get(8)?;
        let pool_withdraw_queue = accounts.get(9)?;
        let serum_program = accounts.get(10)?;
        let serum_market = accounts.get(11)?;
        let serum_coin_vault_account = accounts.get(12)?;
        let serum_pc_vault_account = accounts.get(13)?;
        let serum_vault_signer = accounts.get(14)?;
        let serum_req_q = accounts.get(15)?;
        let serum_event_q = accounts.get(16)?;
        let serum_bids = accounts.get(17)?;
        let serum_asks = accounts.get(18)?;
        let serum_fee_discount = accounts.get(19)?;
        let referrer_pc_account = accounts.get(20)?;

        Some(MonitorStepInstructionAccounts {
            token_program: token_program.pubkey,
            rent: rent.pubkey,
            clock: clock.pubkey,
            amm: amm.pubkey,
            amm_authority: amm_authority.pubkey,
            amm_open_orders: amm_open_orders.pubkey,
            amm_target_orders: amm_target_orders.pubkey,
            pool_coin_token_account: pool_coin_token_account.pubkey,
            pool_pc_token_account: pool_pc_token_account.pubkey,
            pool_withdraw_queue: pool_withdraw_queue.pubkey,
            serum_program: serum_program.pubkey,
            serum_market: serum_market.pubkey,
            serum_coin_vault_account: serum_coin_vault_account.pubkey,
            serum_pc_vault_account: serum_pc_vault_account.pubkey,
            serum_vault_signer: serum_vault_signer.pubkey,
            serum_req_q: serum_req_q.pubkey,
            serum_event_q: serum_event_q.pubkey,
            serum_bids: serum_bids.pubkey,
            serum_asks: serum_asks.pubkey,
            serum_fee_discount: serum_fee_discount.pubkey,
            referrer_pc_account: referrer_pc_account.pubkey,
        })
    }
}
