use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x1d9efcbf0a53db63")]
pub struct UpdateConfig {
    pub param: u8,
    pub value: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateConfigInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateConfig {
    type ArrangedAccounts = UpdateConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, global_config, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateConfigInstructionAccounts {
            owner: owner.pubkey,
            global_config: global_config.pubkey,
        })
    }
}
