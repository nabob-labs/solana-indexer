use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1de150b222d927b894")]
pub struct BondingCurveDeployedEvent {
    pub mint: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
}
