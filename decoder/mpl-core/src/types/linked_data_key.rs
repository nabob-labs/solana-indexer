use {
    super::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum LinkedDataKey {
    LinkedLifecycleHook(solana_pubkey::Pubkey),
    LinkedAppData(Authority),
}
