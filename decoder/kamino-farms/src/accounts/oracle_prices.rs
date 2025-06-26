use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x598076dd0648b492")]
pub struct OraclePrices {
    pub oracle_mappings: solana_pubkey::Pubkey,
    pub prices: [DatedPrice; 512],
}
