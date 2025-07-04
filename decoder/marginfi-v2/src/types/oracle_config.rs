use {
    super::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct OracleConfig {
    pub setup: OracleSetup,
    pub keys: [solana_pubkey::Pubkey; 5],
}
