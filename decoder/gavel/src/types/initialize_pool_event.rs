use super::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitializePoolEvent {
    pub lp_fee_in_bps: u64,
    pub protocol_fee_in_pct: u64,
    pub fee_recipient_params: [ProtocolFeeRecipientParams; 3],
}
