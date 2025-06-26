use super::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PoolHeader {
    pub sequence_number: u64,
    pub base_params: TokenParams,
    pub quote_params: TokenParams,
    pub fee_recipients: ProtocolFeeRecipients,
    pub swap_sequence_number: u64,
    pub padding: [u64; 12],
}
