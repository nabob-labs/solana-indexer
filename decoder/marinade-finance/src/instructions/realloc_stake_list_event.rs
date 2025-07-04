use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1dc18110f3b183f817")]
pub struct ReallocStakeListEvent {
    pub state: solana_pubkey::Pubkey,
    pub count: u32,
    pub new_capacity: u32,
}
