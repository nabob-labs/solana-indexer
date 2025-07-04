use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d76634d0ae2010157")]
pub struct EvtClaimPartnerFeeEvent {
    pub pool: solana_pubkey::Pubkey,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}
