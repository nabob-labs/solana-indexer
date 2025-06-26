use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d3a35edb2ee99559c")]
pub struct SetValidatorScoreEvent {
    pub state: solana_pubkey::Pubkey,
    pub validator: solana_pubkey::Pubkey,
    pub index: u32,
    pub score_change: U32ValueChange,
}
