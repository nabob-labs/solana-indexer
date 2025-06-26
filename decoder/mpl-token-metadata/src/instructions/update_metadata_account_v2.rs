use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0f")]
pub struct UpdateMetadataAccountV2 {
    pub update_metadata_account_args_v2: UpdateMetadataAccountArgsV2,
}

pub struct UpdateMetadataAccountV2InstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateMetadataAccountV2 {
    type ArrangedAccounts = UpdateMetadataAccountV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, update_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateMetadataAccountV2InstructionAccounts {
            metadata: metadata.pubkey,
            update_authority: update_authority.pubkey,
        })
    }
}
