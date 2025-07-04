use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ProtocolFeeRecipientParams {
    pub recipient: solana_pubkey::Pubkey,
    pub shares: u64,
}
