use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x07")]
pub struct SignMetadata {}

pub struct SignMetadataInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SignMetadata {
    type ArrangedAccounts = SignMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, creator, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SignMetadataInstructionAccounts {
            metadata: metadata.pubkey,
            creator: creator.pubkey,
        })
    }
}
