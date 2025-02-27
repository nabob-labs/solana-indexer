use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x1d")]
pub struct WriteCollectionExternalPluginAdapterDataV1 {
    pub write_collection_external_plugin_adapter_data_v1_args:
        WriteCollectionExternalPluginAdapterDataV1Args,
}

pub struct WriteCollectionExternalPluginAdapterDataV1InstructionAccounts {
    pub collection: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub buffer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for WriteCollectionExternalPluginAdapterDataV1 {
    type ArrangedAccounts = WriteCollectionExternalPluginAdapterDataV1InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [collection, payer, authority, buffer, system_program, log_wrapper, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(
            WriteCollectionExternalPluginAdapterDataV1InstructionAccounts {
                collection: collection.pubkey,
                payer: payer.pubkey,
                authority: authority.pubkey,
                buffer: buffer.pubkey,
                system_program: system_program.pubkey,
                log_wrapper: log_wrapper.pubkey,
            },
        )
    }
}
