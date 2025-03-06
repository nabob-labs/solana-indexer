
use super::super::types::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0x2c")]
pub struct Delegate{
    pub delegate_args: DelegateArgs,
}

pub struct DelegateInstructionAccounts {
    pub delegate_record: solana_sdk::pubkey::Pubkey,
    pub delegate: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub token_record: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub token: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
    pub spl_token_program: solana_sdk::pubkey::Pubkey,
    pub authorization_rules_program: solana_sdk::pubkey::Pubkey,
    pub authorization_rules: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Delegate {
    type ArrangedAccounts = DelegateInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let delegate_record = accounts.get(0)?;
        let delegate = accounts.get(1)?;
        let metadata = accounts.get(2)?;
        let master_edition = accounts.get(3)?;
        let token_record = accounts.get(4)?;
        let mint = accounts.get(5)?;
        let token = accounts.get(6)?;
        let authority = accounts.get(7)?;
        let payer = accounts.get(8)?;
        let system_program = accounts.get(9)?;
        let sysvar_instructions = accounts.get(10)?;
        let spl_token_program = accounts.get(11)?;
        let authorization_rules_program = accounts.get(12)?;
        let authorization_rules = accounts.get(13)?;

        Some(DelegateInstructionAccounts {
            delegate_record: delegate_record.pubkey,
            delegate: delegate.pubkey,
            metadata: metadata.pubkey,
            master_edition: master_edition.pubkey,
            token_record: token_record.pubkey,
            mint: mint.pubkey,
            token: token.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
            spl_token_program: spl_token_program.pubkey,
            authorization_rules_program: authorization_rules_program.pubkey,
            authorization_rules: authorization_rules.pubkey,
        })
    }
}