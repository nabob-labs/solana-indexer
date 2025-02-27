use {
    super::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum TransferArgs {
    V1 {
        amount: u64,
        authorization_data: Option<AuthorizationData>,
    },
}
