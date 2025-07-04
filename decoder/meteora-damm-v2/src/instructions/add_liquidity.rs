use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xb59d59438fb63448")]
pub struct AddLiquidity {
    pub params: AddLiquidityParameters,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AddLiquidityInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub token_a_account: solana_pubkey::Pubkey,
    pub token_b_account: solana_pubkey::Pubkey,
    pub token_a_vault: solana_pubkey::Pubkey,
    pub token_b_vault: solana_pubkey::Pubkey,
    pub token_a_mint: solana_pubkey::Pubkey,
    pub token_b_mint: solana_pubkey::Pubkey,
    pub position_nft_account: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub token_a_program: solana_pubkey::Pubkey,
    pub token_b_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for AddLiquidity {
    type ArrangedAccounts = AddLiquidityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, position, token_a_account, token_b_account, token_a_vault, token_b_vault, token_a_mint, token_b_mint, position_nft_account, owner, token_a_program, token_b_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(AddLiquidityInstructionAccounts {
            pool: pool.pubkey,
            position: position.pubkey,
            token_a_account: token_a_account.pubkey,
            token_b_account: token_b_account.pubkey,
            token_a_vault: token_a_vault.pubkey,
            token_b_vault: token_b_vault.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            position_nft_account: position_nft_account.pubkey,
            owner: owner.pubkey,
            token_a_program: token_a_program.pubkey,
            token_b_program: token_b_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
