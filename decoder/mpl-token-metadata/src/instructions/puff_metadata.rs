use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0e")]
pub struct PuffMetadata {}

pub struct PuffMetadataInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for PuffMetadata {
    type ArrangedAccounts = PuffMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, _remaining @ ..] = accounts else {
            return None;
        };

        Some(PuffMetadataInstructionAccounts {
            metadata: metadata.pubkey,
        })
    }
}
