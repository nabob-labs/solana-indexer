use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x3ec7514e210dec3d")]
pub struct MarginfiGroupConfigure {
    pub config: GroupConfig,
}

pub struct MarginfiGroupConfigureInstructionAccounts {
    pub marginfi_group: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for MarginfiGroupConfigure {
    type ArrangedAccounts = MarginfiGroupConfigureInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_group, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(MarginfiGroupConfigureInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            admin: admin.pubkey,
        })
    }
}
