use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0deca4ad6afda4b9")]
pub struct ConfigInit {
    pub data: ConfigParams,
}

#[derive(Debug)]
pub struct ConfigInitInstructionAccounts {
    pub config_authority: solana_pubkey::Pubkey,
    pub config_account: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ConfigInit {
    type ArrangedAccounts = ConfigInitInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [config_authority, config_account, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ConfigInitInstructionAccounts {
            config_authority: config_authority.pubkey,
            config_account: config_account.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
