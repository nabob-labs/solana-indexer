use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d3b8a4c448a83b043")]
pub struct GoToABin {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub from_bin_id: i32,
    pub to_bin_id: i32,
}
