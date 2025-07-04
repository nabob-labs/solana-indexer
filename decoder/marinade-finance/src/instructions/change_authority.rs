use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x326a426863769158")]
pub struct ChangeAuthority {
    pub data: ChangeAuthorityData,
}

pub struct ChangeAuthorityInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub admin_authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ChangeAuthority {
    type ArrangedAccounts = ChangeAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ChangeAuthorityInstructionAccounts {
            state: state.pubkey,
            admin_authority: admin_authority.pubkey,
        })
    }
}
