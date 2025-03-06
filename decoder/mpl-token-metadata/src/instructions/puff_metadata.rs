

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0x0e")]
pub struct PuffMetadata{
}

pub struct PuffMetadataInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for PuffMetadata {
    type ArrangedAccounts = PuffMetadataInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;

        Some(PuffMetadataInstructionAccounts {
            metadata: metadata.pubkey,
        })
    }
}