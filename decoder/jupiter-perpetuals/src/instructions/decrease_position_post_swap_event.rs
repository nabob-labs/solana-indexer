use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d17d210e962f55952")]
pub struct DecreasePositionPostSwapEvent {
    pub position_request_key: solana_sdk::pubkey::Pubkey,
    pub swap_amount: u64,
    pub jupiter_minimum_out: Option<u64>,
}
