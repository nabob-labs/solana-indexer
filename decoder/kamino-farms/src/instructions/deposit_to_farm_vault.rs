use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x83a6405e6cd572b7")]
pub struct DepositToFarmVault {
    pub amount: u64,
}

pub struct DepositToFarmVaultInstructionAccounts {
    pub depositor: solana_pubkey::Pubkey,
    pub farm_state: solana_pubkey::Pubkey,
    pub farm_vault: solana_pubkey::Pubkey,
    pub depositor_ata: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for DepositToFarmVault {
    type ArrangedAccounts = DepositToFarmVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [depositor, farm_state, farm_vault, depositor_ata, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositToFarmVaultInstructionAccounts {
            depositor: depositor.pubkey,
            farm_state: farm_state.pubkey,
            farm_vault: farm_vault.pubkey,
            depositor_ata: depositor_ata.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
