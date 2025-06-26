use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize)]
#[indexer(discriminator = "0x74d2bb77c4c43489")]
pub struct PoolAccount {
    pub pool_header: PoolHeader,
    pub amm: Amm,
}
