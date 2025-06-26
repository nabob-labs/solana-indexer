use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xc541074997698569")]
pub struct GetMinimumDelegation {}

pub struct GetMinimumDelegationInstructionAccounts {}

impl solana_indexer_core::deserialize::ArrangeAccounts for GetMinimumDelegation {
    type ArrangedAccounts = GetMinimumDelegationInstructionAccounts;

    fn arrange_accounts(
        _accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        Some(GetMinimumDelegationInstructionAccounts {})
    }
}
