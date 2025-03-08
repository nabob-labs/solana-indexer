use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0712d3294c53733d")]
pub struct AuthorizeWithSeed {
    pub new_authority: solana_sdk::pubkey::Pubkey,
    pub stake_authorize: StakeAuthorize,
    pub authority_seed: String,
    pub authority_owner: solana_sdk::pubkey::Pubkey,
}

pub struct AuthorizeWithSeedInstructionAccounts {
    pub stake: solana_sdk::pubkey::Pubkey,
    pub authority_base: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for AuthorizeWithSeed {
    type ArrangedAccounts = AuthorizeWithSeedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [stake, authority_base, clock, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AuthorizeWithSeedInstructionAccounts {
            stake: stake.pubkey,
            authority_base: authority_base.pubkey,
            clock: clock.pubkey,
        })
    }
}
