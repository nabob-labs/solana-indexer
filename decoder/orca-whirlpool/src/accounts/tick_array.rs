 
use solana_indexer_core::{borsh, IndexerDeserialize};
use super::super::types::*;

#[derive(IndexerDeserialize, Debug)]
 

#[indexer(discriminator = "0x4561bdbe6e0742bb")]
pub struct TickArray { 
        pub start_tick_index: i32, 
        pub ticks: [Tick; 88], 
        pub whirlpool: solana_sdk::pubkey::Pubkey, 
}
