use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x17")]
pub struct AddCollectionExternalPluginAdapterV1 {
    pub add_collection_external_plugin_adapter_v1_args: AddCollectionExternalPluginAdapterV1Args,
}

pub struct AddCollectionExternalPluginAdapterV1InstructionAccounts {
    pub collection: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for AddCollectionExternalPluginAdapterV1 {
    type ArrangedAccounts = AddCollectionExternalPluginAdapterV1InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [collection, payer, authority, system_program, log_wrapper, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(AddCollectionExternalPluginAdapterV1InstructionAccounts {
            collection: collection.pubkey,
            payer: payer.pubkey,
            authority: authority.pubkey,
            system_program: system_program.pubkey,
            log_wrapper: log_wrapper.pubkey,
        })
    }
}
