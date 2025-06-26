use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum UpdateGlobalConfigValue {
    Bool(bool),
    U16(u16),
    U64(u64),
    Pubkey(solana_pubkey::Pubkey),
}
