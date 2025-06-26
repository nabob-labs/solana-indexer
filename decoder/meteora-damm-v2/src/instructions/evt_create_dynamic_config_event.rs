use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1de7c50da4f8d58598")]
pub struct EvtCreateDynamicConfigEvent {
    pub config: solana_pubkey::Pubkey,
    pub pool_creator_authority: solana_pubkey::Pubkey,
    pub index: u64,
}
