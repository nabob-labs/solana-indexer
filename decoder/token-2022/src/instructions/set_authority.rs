use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x06")]
pub struct SetAuthority {
    pub authority_type: AuthorityType,
    pub new_authority: Option<solana_pubkey::Pubkey>,
}

pub struct SetAuthorityInstructionAccounts {
    pub owned: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetAuthority {
    type ArrangedAccounts = SetAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owned, owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetAuthorityInstructionAccounts {
            owned: owned.pubkey,
            owner: owner.pubkey,
        })
    }
}
