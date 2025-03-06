
use super::super::types::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0x2a")]
pub struct Create{
    pub create_args: CreateArgs,
}

pub struct CreateInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
    pub spl_token_program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Create {
    type ArrangedAccounts = CreateInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;
        let master_edition = accounts.get(1)?;
        let mint = accounts.get(2)?;
        let authority = accounts.get(3)?;
        let payer = accounts.get(4)?;
        let update_authority = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let sysvar_instructions = accounts.get(7)?;
        let spl_token_program = accounts.get(8)?;

        Some(CreateInstructionAccounts {
            metadata: metadata.pubkey,
            master_edition: master_edition.pubkey,
            mint: mint.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            update_authority: update_authority.pubkey,
            system_program: system_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
            spl_token_program: spl_token_program.pubkey,
        })
    }
}