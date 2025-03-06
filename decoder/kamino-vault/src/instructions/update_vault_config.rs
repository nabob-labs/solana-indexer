
use super::super::types::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0x7a0315de9effee9d")]
pub struct UpdateVaultConfig{
    pub entry: VaultConfigField,
    pub data: Vec<u8>,
}

pub struct UpdateVaultConfigInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub vault_state: solana_sdk::pubkey::Pubkey,
    pub klend_program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateVaultConfig {
    type ArrangedAccounts = UpdateVaultConfigInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let admin_authority = accounts.get(0)?;
        let vault_state = accounts.get(1)?;
        let klend_program = accounts.get(2)?;

        Some(UpdateVaultConfigInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            vault_state: vault_state.pubkey,
            klend_program: klend_program.pubkey,
        })
    }
}