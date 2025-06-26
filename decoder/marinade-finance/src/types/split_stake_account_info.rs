use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SplitStakeAccountInfo {
    pub account: solana_pubkey::Pubkey,
    pub index: u32,
}
