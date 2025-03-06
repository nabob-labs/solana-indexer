

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0x00")]
pub struct CreateMetadataAccount{
}

pub struct CreateMetadataAccountInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub mint_authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CreateMetadataAccount {
    type ArrangedAccounts = CreateMetadataAccountInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;
        let mint = accounts.get(1)?;
        let mint_authority = accounts.get(2)?;
        let payer = accounts.get(3)?;
        let update_authority = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let rent = accounts.get(6)?;

        Some(CreateMetadataAccountInstructionAccounts {
            metadata: metadata.pubkey,
            mint: mint.pubkey,
            mint_authority: mint_authority.pubkey,
            payer: payer.pubkey,
            update_authority: update_authority.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}