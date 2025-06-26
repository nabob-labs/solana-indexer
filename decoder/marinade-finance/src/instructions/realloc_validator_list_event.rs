use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d46bff2a4389c820d")]
pub struct ReallocValidatorListEvent {
    pub state: solana_pubkey::Pubkey,
    pub count: u32,
    pub new_capacity: u32,
}
