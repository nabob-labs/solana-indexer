use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xdc3bcfec6cfa2f64")]
pub struct Init {
    pub params: InitParams,
}

pub struct InitInstructionAccounts {
    pub upgrade_authority: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub perpetuals_program: solana_sdk::pubkey::Pubkey,
    pub perpetuals_program_data: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Init {
    type ArrangedAccounts = InitInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [upgrade_authority, admin, transfer_authority, perpetuals, perpetuals_program, perpetuals_program_data, system_program, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitInstructionAccounts {
            upgrade_authority: upgrade_authority.pubkey,
            admin: admin.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            perpetuals_program: perpetuals_program.pubkey,
            perpetuals_program_data: perpetuals_program_data.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
