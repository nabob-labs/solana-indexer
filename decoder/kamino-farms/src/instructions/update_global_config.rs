use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xa45482bd6f3afac8")]
pub struct UpdateGlobalConfig {
    pub mode: u8,
    pub value: [u8; 32],
}

pub struct UpdateGlobalConfigInstructionAccounts {
    pub global_admin: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateGlobalConfig {
    type ArrangedAccounts = UpdateGlobalConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [global_admin, global_config, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateGlobalConfigInstructionAccounts {
            global_admin: global_admin.pubkey,
            global_config: global_config.pubkey,
        })
    }
}
