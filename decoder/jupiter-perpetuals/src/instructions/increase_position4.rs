use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x439335172b391043")]
pub struct IncreasePosition4 {
    pub params: IncreasePosition4Params,
}

pub struct IncreasePosition4InstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub position_request: solana_sdk::pubkey::Pubkey,
    pub position_request_ata: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub custody_doves_price_account: solana_sdk::pubkey::Pubkey,
    pub custody_pythnet_price_account: solana_sdk::pubkey::Pubkey,
    pub collateral_custody: solana_sdk::pubkey::Pubkey,
    pub collateral_custody_doves_price_account: solana_sdk::pubkey::Pubkey,
    pub collateral_custody_pythnet_price_account: solana_sdk::pubkey::Pubkey,
    pub collateral_custody_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for IncreasePosition4 {
    type ArrangedAccounts = IncreasePosition4InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, perpetuals, pool, position_request, position_request_ata, position, custody, custody_doves_price_account, custody_pythnet_price_account, collateral_custody, collateral_custody_doves_price_account, collateral_custody_pythnet_price_account, collateral_custody_token_account, token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(IncreasePosition4InstructionAccounts {
            keeper: keeper.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            position_request: position_request.pubkey,
            position_request_ata: position_request_ata.pubkey,
            position: position.pubkey,
            custody: custody.pubkey,
            custody_doves_price_account: custody_doves_price_account.pubkey,
            custody_pythnet_price_account: custody_pythnet_price_account.pubkey,
            collateral_custody: collateral_custody.pubkey,
            collateral_custody_doves_price_account: collateral_custody_doves_price_account.pubkey,
            collateral_custody_pythnet_price_account: collateral_custody_pythnet_price_account
                .pubkey,
            collateral_custody_token_account: collateral_custody_token_account.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
