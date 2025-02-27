use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum RuleSet {
    None,
    ProgramAllowList(Vec<solana_sdk::pubkey::Pubkey>),
    ProgramDenyList(Vec<solana_sdk::pubkey::Pubkey>),
}
