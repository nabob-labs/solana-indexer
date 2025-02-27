use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xd7e4a6e45464567b")]
pub struct UpdateTokenMetadataUpdateAuthority {
    pub new_update_authority: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct UpdateTokenMetadataUpdateAuthorityInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateTokenMetadataUpdateAuthority {
    type ArrangedAccounts = UpdateTokenMetadataUpdateAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, update_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateTokenMetadataUpdateAuthorityInstructionAccounts {
            metadata: metadata.pubkey,
            update_authority: update_authority.pubkey,
        })
    }
}
