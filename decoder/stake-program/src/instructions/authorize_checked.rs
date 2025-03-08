use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x9361431ae66b2df2")]
pub struct AuthorizeChecked {
    pub stake_authorize: StakeAuthorize,
}

pub struct AuthorizeCheckedInstructionAccounts {
    pub stake: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub new_authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for AuthorizeChecked {
    type ArrangedAccounts = AuthorizeCheckedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [stake, clock, authority, new_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AuthorizeCheckedInstructionAccounts {
            stake: stake.pubkey,
            clock: clock.pubkey,
            authority: authority.pubkey,
            new_authority: new_authority.pubkey,
        })
    }
}
