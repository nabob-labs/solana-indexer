use super::super::types::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0x132c829448382cee")]
pub struct ProxySwap{
    pub data: SwapArgs,
    pub order_id: u64,
}

pub struct ProxySwapInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub source_token_account: solana_sdk::pubkey::Pubkey,
    pub destination_token_account: solana_sdk::pubkey::Pubkey,
    pub source_mint: solana_sdk::pubkey::Pubkey,
    pub destination_mint: solana_sdk::pubkey::Pubkey,
    pub sa_authority: solana_sdk::pubkey::Pubkey,
    pub source_token_sa: solana_sdk::pubkey::Pubkey,
    pub destination_token_sa: solana_sdk::pubkey::Pubkey,
    pub source_token_program: solana_sdk::pubkey::Pubkey,
    pub destination_token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ProxySwap {
    type ArrangedAccounts = ProxySwapInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            payer,
            source_token_account,
            destination_token_account,
            source_mint,
            destination_mint,
            sa_authority,
            source_token_sa,
            destination_token_sa,
            source_token_program,
            destination_token_program,
            associated_token_program,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(ProxySwapInstructionAccounts {
            payer: payer.pubkey,
            source_token_account: source_token_account.pubkey,
            destination_token_account: destination_token_account.pubkey,
            source_mint: source_mint.pubkey,
            destination_mint: destination_mint.pubkey,
            sa_authority: sa_authority.pubkey,
            source_token_sa: source_token_sa.pubkey,
            destination_token_sa: destination_token_sa.pubkey,
            source_token_program: source_token_program.pubkey,
            destination_token_program: destination_token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}