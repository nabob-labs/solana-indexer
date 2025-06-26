use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct BondingCurveDeployedFallbackEvent {
    pub mint: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
}
