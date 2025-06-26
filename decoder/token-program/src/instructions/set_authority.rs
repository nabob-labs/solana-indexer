use {
    crate::types::*,
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

pub struct SetAuthorityAccounts {
    pub account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetAuthority {
    type ArrangedAccounts = SetAuthorityAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, authority, remaining_accounts @ ..] = accounts else {
            return None;
        };

        Some(SetAuthorityAccounts {
            account: account.pubkey,
            authority: authority.pubkey,
            remaining_accounts: remaining_accounts.to_vec(),
        })
    }
}
