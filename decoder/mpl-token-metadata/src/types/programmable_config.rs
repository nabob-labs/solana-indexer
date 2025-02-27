use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum ProgrammableConfig {
    V1 {
        rule_set: Option<solana_sdk::pubkey::Pubkey>,
    },
}
