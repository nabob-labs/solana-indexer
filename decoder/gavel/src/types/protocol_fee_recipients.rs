use super::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ProtocolFeeRecipients {
    pub recipients: [ProtocolFeeRecipient; 3],
    pub padding: [u64; 12],
}
