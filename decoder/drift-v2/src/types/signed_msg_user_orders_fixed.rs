use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SignedMsgUserOrdersFixed {
    pub user_pubkey: solana_sdk::pubkey::Pubkey,
    pub padding: u32,
    pub len: u32,
}
