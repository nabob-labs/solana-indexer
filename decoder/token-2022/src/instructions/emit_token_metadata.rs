use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xfaa6b4fa0d0cb846")]
pub struct EmitTokenMetadata {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

pub struct EmitTokenMetadataInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for EmitTokenMetadata {
    type ArrangedAccounts = EmitTokenMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, _remaining @ ..] = accounts else {
            return None;
        };

        Some(EmitTokenMetadataInstructionAccounts {
            metadata: metadata.pubkey,
        })
    }
}
