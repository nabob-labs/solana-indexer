use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x71d87a83e1d11637")]
pub struct InitializeGlobalConfig {}

pub struct InitializeGlobalConfigInstructionAccounts {
    pub global_admin: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub treasury_vaults_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeGlobalConfig {
    type ArrangedAccounts = InitializeGlobalConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [global_admin, global_config, treasury_vaults_authority, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeGlobalConfigInstructionAccounts {
            global_admin: global_admin.pubkey,
            global_config: global_config.pubkey,
            treasury_vaults_authority: treasury_vaults_authority.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
