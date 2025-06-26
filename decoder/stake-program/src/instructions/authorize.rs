use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xadc166d2db897178")]
pub struct Authorize {
    pub new_authority: solana_pubkey::Pubkey,
    pub stake_authorize: StakeAuthorize,
}

pub struct AuthorizeInstructionAccounts {
    pub stake: solana_pubkey::Pubkey,
    pub clock: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Authorize {
    type ArrangedAccounts = AuthorizeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [stake, clock, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AuthorizeInstructionAccounts {
            stake: stake.pubkey,
            clock: clock.pubkey,
            authority: authority.pubkey,
        })
    }
}
