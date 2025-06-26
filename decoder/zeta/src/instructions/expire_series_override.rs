use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x6816227b56e08246")]
pub struct ExpireSeriesOverride {
    pub args: ExpireSeriesOverrideArgs,
}

pub struct ExpireSeriesOverrideInstructionAccounts {}

impl solana_indexer_core::deserialize::ArrangeAccounts for ExpireSeriesOverride {
    type ArrangedAccounts = ExpireSeriesOverrideInstructionAccounts;

    fn arrange_accounts(
        _accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        Some(ExpireSeriesOverrideInstructionAccounts {})
    }
}
