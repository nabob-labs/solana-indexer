use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Authorized {
    pub staker: solana_sdk::pubkey::Pubkey,
    pub withdrawer: solana_sdk::pubkey::Pubkey,
}
