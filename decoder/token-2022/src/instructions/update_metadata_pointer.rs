use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x27")]
pub struct UpdateMetadataPointer {
    pub metadata_pointer_discriminator: u8,
    pub metadata_address: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct UpdateMetadataPointerInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub metadata_pointer_authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateMetadataPointer {
    type ArrangedAccounts = UpdateMetadataPointerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, metadata_pointer_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateMetadataPointerInstructionAccounts {
            mint: mint.pubkey,
            metadata_pointer_authority: metadata_pointer_authority.pubkey,
        })
    }
}
