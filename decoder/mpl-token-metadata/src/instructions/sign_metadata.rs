

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0x07")]
pub struct SignMetadata{
}

pub struct SignMetadataInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub creator: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SignMetadata {
    type ArrangedAccounts = SignMetadataInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;
        let creator = accounts.get(1)?;

        Some(SignMetadataInstructionAccounts {
            metadata: metadata.pubkey,
            creator: creator.pubkey,
        })
    }
}