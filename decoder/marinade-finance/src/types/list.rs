use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct List {
    pub account: solana_pubkey::Pubkey,
    pub item_size: u32,
    pub count: u32,
    pub reserved1: solana_pubkey::Pubkey,
    pub reserved2: u32,
}
