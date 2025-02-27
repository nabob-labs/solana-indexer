use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AutographSignature {
    pub address: solana_sdk::pubkey::Pubkey,
    pub message: String,
}
