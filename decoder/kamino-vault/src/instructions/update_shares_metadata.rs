use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x9b227aa5f589936b")]
pub struct UpdateSharesMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

pub struct UpdateSharesMetadataInstructionAccounts {
    pub admin_authority: solana_pubkey::Pubkey,
    pub vault_state: solana_pubkey::Pubkey,
    pub base_vault_authority: solana_pubkey::Pubkey,
    pub shares_metadata: solana_pubkey::Pubkey,
    pub metadata_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateSharesMetadata {
    type ArrangedAccounts = UpdateSharesMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin_authority, vault_state, base_vault_authority, shares_metadata, metadata_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(UpdateSharesMetadataInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            vault_state: vault_state.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            shares_metadata: shares_metadata.pubkey,
            metadata_program: metadata_program.pubkey,
        })
    }
}
