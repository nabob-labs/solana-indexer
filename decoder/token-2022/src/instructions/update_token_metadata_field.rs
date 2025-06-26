use {
    super::super::types::*,
    alloc::string::String,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xdde9312db5cadcc8")]
pub struct UpdateTokenMetadataField {
    pub field: TokenMetadataField,
    pub value: String,
}

pub struct UpdateTokenMetadataFieldInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateTokenMetadataField {
    type ArrangedAccounts = UpdateTokenMetadataFieldInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, update_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateTokenMetadataFieldInstructionAccounts {
            metadata: metadata.pubkey,
            update_authority: update_authority.pubkey,
        })
    }
}
