use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x2da269622c15ab7f")]
pub struct ExpireSeries {
    pub settlement_nonce: u8,
}

pub struct ExpireSeriesInstructionAccounts {}

impl solana_indexer_core::deserialize::ArrangeAccounts for ExpireSeries {
    type ArrangedAccounts = ExpireSeriesInstructionAccounts;

    #[allow(irrefutable_let_patterns)]
    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [_remaining @ ..] = accounts else {
            return None;
        };

        Some(ExpireSeriesInstructionAccounts {})
    }
}
