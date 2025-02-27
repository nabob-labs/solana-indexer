use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Creator {
    pub address: solana_sdk::pubkey::Pubkey,
    pub percentage: u8,
}
