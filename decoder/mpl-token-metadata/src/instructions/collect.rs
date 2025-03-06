

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0x36")]
pub struct Collect{
}

pub struct CollectInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub recipient: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Collect {
    type ArrangedAccounts = CollectInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let authority = accounts.get(0)?;
        let recipient = accounts.get(1)?;

        Some(CollectInstructionAccounts {
            authority: authority.pubkey,
            recipient: recipient.pubkey,
        })
    }
}