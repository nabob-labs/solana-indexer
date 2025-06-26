use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xc9cff3724b6f2fbd")]
pub struct CreateConfig {
    pub config_parameters: ConfigParameters,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateConfigInstructionAccounts {
    pub config: solana_pubkey::Pubkey,
    pub fee_claimer: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CreateConfig {
    type ArrangedAccounts = CreateConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [config, fee_claimer, owner, quote_mint, payer, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateConfigInstructionAccounts {
            config: config.pubkey,
            fee_claimer: fee_claimer.pubkey,
            owner: owner.pubkey,
            quote_mint: quote_mint.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
