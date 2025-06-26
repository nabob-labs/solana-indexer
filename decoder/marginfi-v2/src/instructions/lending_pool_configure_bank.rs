use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x79ad9c285d9438ed")]
pub struct LendingPoolConfigureBank {
    pub bank_config_opt: BankConfigOpt,
}

pub struct LendingPoolConfigureBankInstructionAccounts {
    pub marginfi_group: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for LendingPoolConfigureBank {
    type ArrangedAccounts = LendingPoolConfigureBankInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_group, admin, bank, _remaining @ ..] = accounts else {
            return None;
        };

        Some(LendingPoolConfigureBankInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            admin: admin.pubkey,
            bank: bank.pubkey,
        })
    }
}
