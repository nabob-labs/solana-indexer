use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x7aaec5358109a584")]
pub struct ObservationState {
    pub initialized: bool,
    pub recent_epoch: u64,
    pub observation_index: u16,
    pub pool_id: solana_pubkey::Pubkey,
    pub observations: [Observation; 100],
    pub padding: [u64; 4],
}
