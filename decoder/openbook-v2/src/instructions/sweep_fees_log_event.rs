use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1dd2f21a4d5e30ff3d")]
pub struct SweepFeesLogEvent {
    pub market: solana_sdk::pubkey::Pubkey,
    pub amount: u64,
    pub receiver: solana_sdk::pubkey::Pubkey,
}
