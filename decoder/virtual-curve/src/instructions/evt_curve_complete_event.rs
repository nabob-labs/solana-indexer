use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1de5e756549c864b18")]
pub struct EvtCurveCompleteEvent {
    pub pool: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub base_reserve: u64,
    pub quote_reserve: u64,
}
