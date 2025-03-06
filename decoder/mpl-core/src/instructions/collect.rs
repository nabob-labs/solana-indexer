

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0x13")]
pub struct Collect{
}

pub struct CollectInstructionAccounts {
    pub recipient1: solana_sdk::pubkey::Pubkey,
    pub recipient2: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Collect {
    type ArrangedAccounts = CollectInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let recipient1 = accounts.get(0)?;
        let recipient2 = accounts.get(1)?;

        Some(CollectInstructionAccounts {
            recipient1: recipient1.pubkey,
            recipient2: recipient2.pubkey,
        })
    }
}