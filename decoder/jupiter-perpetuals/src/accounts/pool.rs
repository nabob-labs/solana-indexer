use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0xf19a6d0411b16dbc")]
pub struct Pool {
    pub name: String,
    pub custodies: Vec<solana_pubkey::Pubkey>,
    pub aum_usd: u128,
    pub limit: Limit,
    pub fees: Fees,
    pub pool_apr: PoolApr,
    pub max_request_execution_sec: i64,
    pub bump: u8,
    pub lp_token_bump: u8,
    pub inception_time: i64,
}
