use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct BondingCurveVaultClosedEvent {
    pub mint: solana_pubkey::Pubkey,
    pub recipient: solana_pubkey::Pubkey,
    pub amount: u64,
}
