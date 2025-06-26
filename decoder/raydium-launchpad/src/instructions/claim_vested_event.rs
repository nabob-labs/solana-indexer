use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d15c2725778d3e220")]
pub struct ClaimVestedEvent {
    pub pool_state: solana_pubkey::Pubkey,
    pub beneficiary: solana_pubkey::Pubkey,
    pub claim_amount: u64,
}
