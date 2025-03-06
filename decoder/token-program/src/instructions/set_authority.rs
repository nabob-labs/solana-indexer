use crate::types::*;
use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x06")]
pub struct SetAuthority {
    pub authority_type: AuthorityType,
    pub new_authority: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct SetAuthorityAccounts {
    pub account: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetAuthority {
    type ArrangedAccounts = SetAuthorityAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let account = accounts.get(0)?;
        let authority = accounts.get(1)?;

        Some(SetAuthorityAccounts {
            account: account.pubkey,
            authority: authority.pubkey,
            remaining_accounts: accounts.get(2..).unwrap_or_default().to_vec(),
        })
    }
}
