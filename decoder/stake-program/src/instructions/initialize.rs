use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xafaf6d1f0d989bed")]
pub struct Initialize {
    pub authorized: Authorized,
    pub lockup: Lockup,
}

pub struct InitializeInstructionAccounts {
    pub stake: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [stake, rent, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeInstructionAccounts {
            stake: stake.pubkey,
            rent: rent.pubkey,
        })
    }
}
