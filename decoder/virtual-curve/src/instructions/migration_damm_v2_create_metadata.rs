use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x6dbd1324c3b7de52")]
pub struct MigrationDammV2CreateMetadata {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MigrationDammV2CreateMetadataInstructionAccounts {
    pub virtual_pool: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub migration_metadata: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for MigrationDammV2CreateMetadata {
    type ArrangedAccounts = MigrationDammV2CreateMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [virtual_pool, config, migration_metadata, payer, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(MigrationDammV2CreateMetadataInstructionAccounts {
            virtual_pool: virtual_pool.pubkey,
            config: config.pubkey,
            migration_metadata: migration_metadata.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
