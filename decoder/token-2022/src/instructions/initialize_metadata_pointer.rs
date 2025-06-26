use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x27")]
pub struct InitializeMetadataPointer {
    pub metadata_pointer_discriminator: u8,
    pub authority: Option<solana_pubkey::Pubkey>,
    pub metadata_address: Option<solana_pubkey::Pubkey>,
}

pub struct InitializeMetadataPointerInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeMetadataPointer {
    type ArrangedAccounts = InitializeMetadataPointerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeMetadataPointerInstructionAccounts { mint: mint.pubkey })
    }
}
