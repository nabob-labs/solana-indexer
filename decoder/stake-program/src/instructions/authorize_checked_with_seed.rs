use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0ee69aa5e1d1c2d2")]
pub struct AuthorizeCheckedWithSeed {
    pub stake_authorize: StakeAuthorize,
    pub authority_seed: String,
    pub authority_owner: solana_sdk::pubkey::Pubkey,
}

pub struct AuthorizeCheckedWithSeedInstructionAccounts {
    pub stake: solana_sdk::pubkey::Pubkey,
    pub authority_base: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub new_authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for AuthorizeCheckedWithSeed {
    type ArrangedAccounts = AuthorizeCheckedWithSeedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [stake, authority_base, clock, new_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AuthorizeCheckedWithSeedInstructionAccounts {
            stake: stake.pubkey,
            authority_base: authority_base.pubkey,
            clock: clock.pubkey,
            new_authority: new_authority.pubkey,
        })
    }
}
