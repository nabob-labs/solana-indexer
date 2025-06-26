use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d6d6f1cdd86c3e6cb")]
pub struct EvtProtocolWithdrawSurplusEvent {
    pub pool: solana_pubkey::Pubkey,
    pub surplus_amount: u64,
}
