use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x3c9624db61808b99")]
pub struct TickArrayBitmapExtension {
    pub pool_id: solana_pubkey::Pubkey,
    pub positive_tick_array_bitmap: [[u64; 8]; 14],
    pub negative_tick_array_bitmap: [[u64; 8]; 14],
}
