use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d815bbc03f634b9f9")]
pub struct EvtInitializeRewardEvent {
    pub pool: solana_pubkey::Pubkey,
    pub reward_mint: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub reward_index: u8,
    pub reward_duration: u64,
}
