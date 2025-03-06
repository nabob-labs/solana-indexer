
use super::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum OracleValidation {
    Uninitialized,
    V1
                {
                    create: ExternalValidationResult,
                    transfer: ExternalValidationResult,
                    burn: ExternalValidationResult,
                    update: ExternalValidationResult,
                }
    ,
}


