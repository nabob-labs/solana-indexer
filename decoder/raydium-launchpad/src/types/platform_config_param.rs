use super::*;

use alloc::string::String;
use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum PlatformConfigParam {
    FeeWallet(solana_pubkey::Pubkey),
    NFTWallet(solana_pubkey::Pubkey),
    MigrateNftInfo(MigrateNftInfo),
    FeeRate(u64),
    Name(String),
    Web(String),
    Img(String),
}
