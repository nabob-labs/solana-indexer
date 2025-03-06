use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x506f7c7137ed1205")]
pub struct BinArrayBitmapExtension {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub positive_bin_array_bitmap: [[u64; 8]; 12],
    pub negative_bin_array_bitmap: [[u64; 8]; 12],
}
