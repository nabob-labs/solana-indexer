use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xcc02c391359191cd")]
pub struct RemoveLiquidityByRange2 {
    pub from_bin_id: i32,
    pub to_bin_id: i32,
    pub bps_to_remove: u16,
    pub remaining_accounts_info: RemainingAccountsInfo,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RemoveLiquidityByRange2InstructionAccounts {
    pub position: solana_pubkey::Pubkey,
    pub lb_pair: solana_pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_pubkey::Pubkey,
    pub user_token_x: solana_pubkey::Pubkey,
    pub user_token_y: solana_pubkey::Pubkey,
    pub reserve_x: solana_pubkey::Pubkey,
    pub reserve_y: solana_pubkey::Pubkey,
    pub token_x_mint: solana_pubkey::Pubkey,
    pub token_y_mint: solana_pubkey::Pubkey,
    pub sender: solana_pubkey::Pubkey,
    pub token_x_program: solana_pubkey::Pubkey,
    pub token_y_program: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for RemoveLiquidityByRange2 {
    type ArrangedAccounts = RemoveLiquidityByRange2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [position, lb_pair, bin_array_bitmap_extension, user_token_x, user_token_y, reserve_x, reserve_y, token_x_mint, token_y_mint, sender, token_x_program, token_y_program, memo_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RemoveLiquidityByRange2InstructionAccounts {
            position: position.pubkey,
            lb_pair: lb_pair.pubkey,
            bin_array_bitmap_extension: bin_array_bitmap_extension.pubkey,
            user_token_x: user_token_x.pubkey,
            user_token_y: user_token_y.pubkey,
            reserve_x: reserve_x.pubkey,
            reserve_y: reserve_y.pubkey,
            token_x_mint: token_x_mint.pubkey,
            token_y_mint: token_y_mint.pubkey,
            sender: sender.pubkey,
            token_x_program: token_x_program.pubkey,
            token_y_program: token_y_program.pubkey,
            memo_program: memo_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
