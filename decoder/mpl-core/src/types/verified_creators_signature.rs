

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct VerifiedCreatorsSignature {
    pub address: solana_sdk::pubkey::Pubkey,
    pub verified: bool,
}
