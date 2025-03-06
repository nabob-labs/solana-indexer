
use super::super::types::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0x22")]
pub struct SetCollectionSize{
    pub set_collection_size_args: SetCollectionSizeArgs,
}

pub struct SetCollectionSizeInstructionAccounts {
    pub collection_metadata: solana_sdk::pubkey::Pubkey,
    pub collection_authority: solana_sdk::pubkey::Pubkey,
    pub collection_mint: solana_sdk::pubkey::Pubkey,
    pub collection_authority_record: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetCollectionSize {
    type ArrangedAccounts = SetCollectionSizeInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let collection_metadata = accounts.get(0)?;
        let collection_authority = accounts.get(1)?;
        let collection_mint = accounts.get(2)?;
        let collection_authority_record = accounts.get(3)?;

        Some(SetCollectionSizeInstructionAccounts {
            collection_metadata: collection_metadata.pubkey,
            collection_authority: collection_authority.pubkey,
            collection_mint: collection_mint.pubkey,
            collection_authority_record: collection_authority_record.pubkey,
        })
    }
}