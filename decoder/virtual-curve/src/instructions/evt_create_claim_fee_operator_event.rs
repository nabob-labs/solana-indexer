use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d1506997844741cb1")]
pub struct EvtCreateClaimFeeOperatorEvent {
    pub operator: solana_pubkey::Pubkey,
}
