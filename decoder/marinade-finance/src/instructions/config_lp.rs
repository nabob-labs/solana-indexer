use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0a18a8775630e111")]
pub struct ConfigLp {
    pub params: ConfigLpParams,
}

pub struct ConfigLpInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub admin_authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ConfigLp {
    type ArrangedAccounts = ConfigLpInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ConfigLpInstructionAccounts {
            state: state.pubkey,
            admin_authority: admin_authority.pubkey,
        })
    }
}
