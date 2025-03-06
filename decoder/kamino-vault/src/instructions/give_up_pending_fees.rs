

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0xb1c878866ed99351")]
pub struct GiveUpPendingFees{
    pub max_amount_to_give_up: u64,
}

pub struct GiveUpPendingFeesInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub vault_state: solana_sdk::pubkey::Pubkey,
    pub klend_program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for GiveUpPendingFees {
    type ArrangedAccounts = GiveUpPendingFeesInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let admin_authority = accounts.get(0)?;
        let vault_state = accounts.get(1)?;
        let klend_program = accounts.get(2)?;

        Some(GiveUpPendingFeesInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            vault_state: vault_state.pubkey,
            klend_program: klend_program.pubkey,
        })
    }
}