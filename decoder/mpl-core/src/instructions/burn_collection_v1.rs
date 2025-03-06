
use super::super::types::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0x0d")]
pub struct BurnCollectionV1{
    pub burn_collection_v1_args: BurnCollectionV1Args,
}

pub struct BurnCollectionV1InstructionAccounts {
    pub collection: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for BurnCollectionV1 {
    type ArrangedAccounts = BurnCollectionV1InstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let collection = accounts.get(0)?;
        let payer = accounts.get(1)?;
        let authority = accounts.get(2)?;
        let log_wrapper = accounts.get(3)?;

        Some(BurnCollectionV1InstructionAccounts {
            collection: collection.pubkey,
            payer: payer.pubkey,
            authority: authority.pubkey,
            log_wrapper: log_wrapper.pubkey,
        })
    }
}