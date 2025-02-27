use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum LoanTerms {
    Time {
        start: i64,
        duration: u64,
        total_owed_lamports: u64,
    },
}
