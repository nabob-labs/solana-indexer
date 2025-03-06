
use super::super::types::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0x11")]
pub struct CreateMasterEditionV3{
    pub create_master_edition_args: CreateMasterEditionArgs,
}

pub struct CreateMasterEditionV3InstructionAccounts {
    pub edition: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub mint_authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CreateMasterEditionV3 {
    type ArrangedAccounts = CreateMasterEditionV3InstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let edition = accounts.get(0)?;
        let mint = accounts.get(1)?;
        let update_authority = accounts.get(2)?;
        let mint_authority = accounts.get(3)?;
        let payer = accounts.get(4)?;
        let metadata = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let rent = accounts.get(8)?;

        Some(CreateMasterEditionV3InstructionAccounts {
            edition: edition.pubkey,
            mint: mint.pubkey,
            update_authority: update_authority.pubkey,
            mint_authority: mint_authority.pubkey,
            payer: payer.pubkey,
            metadata: metadata.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}