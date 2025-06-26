use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x81a9af41b95f2064")]
pub struct PositionBundle {
    pub position_bundle_mint: solana_pubkey::Pubkey,
    pub position_bitmap: [u8; 32],
}
