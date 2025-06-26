use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d96980bb334d2bf7d")]
pub struct CreateVestingEvent {
    pub pool_state: solana_pubkey::Pubkey,
    pub beneficiary: solana_pubkey::Pubkey,
    pub share_amount: u64,
}
