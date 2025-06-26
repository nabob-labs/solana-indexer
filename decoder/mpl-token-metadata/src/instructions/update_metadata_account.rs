use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x01")]
pub struct UpdateMetadataAccount {}

pub struct UpdateMetadataAccountInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateMetadataAccount {
    type ArrangedAccounts = UpdateMetadataAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, update_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateMetadataAccountInstructionAccounts {
            metadata: metadata.pubkey,
            update_authority: update_authority.pubkey,
        })
    }
}
