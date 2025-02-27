use super::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Product {
    pub market: solana_sdk::pubkey::Pubkey,
    pub strike: Strike,
    pub dirty: bool,
    pub kind: Kind,
}
