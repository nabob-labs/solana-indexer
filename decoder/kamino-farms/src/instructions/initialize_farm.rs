use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xfc1cb9acf44a75a5")]
pub struct InitializeFarm {}

pub struct InitializeFarmInstructionAccounts {
    pub farm_admin: solana_pubkey::Pubkey,
    pub farm_state: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub farm_vault: solana_pubkey::Pubkey,
    pub farm_vaults_authority: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeFarm {
    type ArrangedAccounts = InitializeFarmInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [farm_admin, farm_state, global_config, farm_vault, farm_vaults_authority, token_mint, token_program, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeFarmInstructionAccounts {
            farm_admin: farm_admin.pubkey,
            farm_state: farm_state.pubkey,
            global_config: global_config.pubkey,
            farm_vault: farm_vault.pubkey,
            farm_vaults_authority: farm_vaults_authority.pubkey,
            token_mint: token_mint.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
