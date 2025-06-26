use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d3582975c6d399170")]
pub struct SetDelegateLogEvent {
    pub open_orders_account: solana_pubkey::Pubkey,
    pub delegate: Option<solana_pubkey::Pubkey>,
}
