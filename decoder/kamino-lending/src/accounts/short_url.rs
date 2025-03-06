
 
use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
 

#[indexer(discriminator = "0x1c59ae19e27c7ed4")]
pub struct ShortUrl { 
        pub referrer: solana_sdk::pubkey::Pubkey, 
        pub short_url: String, 
}