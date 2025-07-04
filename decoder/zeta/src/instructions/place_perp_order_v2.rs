use super::super::types::*;

use alloc::string::String;
use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xcd5482b43f760acf")]
pub struct PlacePerpOrderV2 {
    pub price: u64,
    pub size: u64,
    pub side: Side,
    pub order_type: OrderType,
    pub client_order_id: Option<u64>,
    pub tag: Option<String>,
    pub tif_offset: Option<u16>,
}

pub struct PlacePerpOrderV2InstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub zeta_group: solana_pubkey::Pubkey,
    pub margin_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub dex_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub serum_authority: solana_pubkey::Pubkey,
    pub greeks: solana_pubkey::Pubkey,
    pub open_orders: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub market_accounts: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub oracle_backup_feed: solana_pubkey::Pubkey,
    pub oracle_backup_program: solana_pubkey::Pubkey,
    pub market_mint: solana_pubkey::Pubkey,
    pub mint_authority: solana_pubkey::Pubkey,
    pub perp_sync_queue: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for PlacePerpOrderV2 {
    type ArrangedAccounts = PlacePerpOrderV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, zeta_group, margin_account, authority, dex_program, token_program, serum_authority, greeks, open_orders, rent, market_accounts, oracle, oracle_backup_feed, oracle_backup_program, market_mint, mint_authority, perp_sync_queue, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(PlacePerpOrderV2InstructionAccounts {
            state: state.pubkey,
            zeta_group: zeta_group.pubkey,
            margin_account: margin_account.pubkey,
            authority: authority.pubkey,
            dex_program: dex_program.pubkey,
            token_program: token_program.pubkey,
            serum_authority: serum_authority.pubkey,
            greeks: greeks.pubkey,
            open_orders: open_orders.pubkey,
            rent: rent.pubkey,
            market_accounts: market_accounts.pubkey,
            oracle: oracle.pubkey,
            oracle_backup_feed: oracle_backup_feed.pubkey,
            oracle_backup_program: oracle_backup_program.pubkey,
            market_mint: market_mint.pubkey,
            mint_authority: mint_authority.pubkey,
            perp_sync_queue: perp_sync_queue.pubkey,
        })
    }
}
