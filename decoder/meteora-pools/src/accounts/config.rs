use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x9b0caae01efacc82")]
pub struct Config {
    pub pool_fees: PoolFees,
    pub activation_duration: u64,
    pub vault_config_key: solana_pubkey::Pubkey,
    pub pool_creator_authority: solana_pubkey::Pubkey,
    pub activation_type: u8,
    pub partner_fee_numerator: u64,
    pub padding: [u8; 219],
}
