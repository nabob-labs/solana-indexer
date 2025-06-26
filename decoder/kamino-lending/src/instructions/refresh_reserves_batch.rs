use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x906e1a67a2ccfc93")]
pub struct RefreshReservesBatch {
    pub skip_price_updates: bool,
}

pub struct RefreshReservesBatchInstructionAccounts {}

impl solana_indexer_core::deserialize::ArrangeAccounts for RefreshReservesBatch {
    type ArrangedAccounts = RefreshReservesBatchInstructionAccounts;

    fn arrange_accounts(
        _accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        Some(RefreshReservesBatchInstructionAccounts {})
    }
}
