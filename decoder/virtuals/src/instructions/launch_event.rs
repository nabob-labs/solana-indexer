use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d1bc12f82735cef5e")]
pub struct LaunchEvent {
    pub vpool: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
}
