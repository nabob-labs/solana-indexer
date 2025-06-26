use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d4b7a9a308c4a7ba3")]
pub struct ClaimFeeEvent {
    pub pool: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub amount: u64,
    pub a_fee: u64,
    pub b_fee: u64,
}
