use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1db9249c52bda4cf4f")]
pub struct BondingCurveVaultClosedEvent {
    pub mint: solana_pubkey::Pubkey,
    pub recipient: solana_pubkey::Pubkey,
    pub amount: u64,
}
