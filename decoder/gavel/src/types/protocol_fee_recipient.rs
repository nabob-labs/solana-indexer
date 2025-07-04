use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ProtocolFeeRecipient {
    pub recipient: solana_pubkey::Pubkey,
    pub shares: u64,
    pub total_accumulated_quote_fees: u64,
    pub collected_quote_fees: u64,
}
