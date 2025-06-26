use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LookupTableMeta {
    pub deactivation_slot: u64,
    pub last_extended_slot: u64,
    pub last_extended_slot_start_index: u8,
    pub authority: Option<solana_pubkey::Pubkey>,
    pub padding: u16,
}
