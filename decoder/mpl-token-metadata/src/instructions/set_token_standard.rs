

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0x23")]
pub struct SetTokenStandard{
}

pub struct SetTokenStandardInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetTokenStandard {
    type ArrangedAccounts = SetTokenStandardInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;
        let update_authority = accounts.get(1)?;
        let mint = accounts.get(2)?;
        let edition = accounts.get(3)?;

        Some(SetTokenStandardInstructionAccounts {
            metadata: metadata.pubkey,
            update_authority: update_authority.pubkey,
            mint: mint.pubkey,
            edition: edition.pubkey,
        })
    }
}