use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x37d5e0a89935c528")]
pub struct LendingPoolUpdateEmissionsParameters {
    pub emissions_flags: Option<u64>,
    pub emissions_rate: Option<u64>,
    pub additional_emissions: Option<u64>,
}

pub struct LendingPoolUpdateEmissionsParametersInstructionAccounts {
    pub marginfi_group: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub emissions_mint: solana_pubkey::Pubkey,
    pub emissions_token_account: solana_pubkey::Pubkey,
    pub emissions_funding_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for LendingPoolUpdateEmissionsParameters {
    type ArrangedAccounts = LendingPoolUpdateEmissionsParametersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_group, admin, bank, emissions_mint, emissions_token_account, emissions_funding_account, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LendingPoolUpdateEmissionsParametersInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            admin: admin.pubkey,
            bank: bank.pubkey,
            emissions_mint: emissions_mint.pubkey,
            emissions_token_account: emissions_token_account.pubkey,
            emissions_funding_account: emissions_funding_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
