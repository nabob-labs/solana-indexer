
use super::super::types::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0x07")]
pub struct UpdateCollectionPluginV1{
    pub update_collection_plugin_v1_args: UpdateCollectionPluginV1Args,
}

pub struct UpdateCollectionPluginV1InstructionAccounts {
    pub collection: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateCollectionPluginV1 {
    type ArrangedAccounts = UpdateCollectionPluginV1InstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let collection = accounts.get(0)?;
        let payer = accounts.get(1)?;
        let authority = accounts.get(2)?;
        let system_program = accounts.get(3)?;
        let log_wrapper = accounts.get(4)?;

        Some(UpdateCollectionPluginV1InstructionAccounts {
            collection: collection.pubkey,
            payer: payer.pubkey,
            authority: authority.pubkey,
            system_program: system_program.pubkey,
            log_wrapper: log_wrapper.pubkey,
        })
    }
}