use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x34170507aa5a6cd5")]
pub struct LockedClmmPositionState {
    pub bump: [u8; 1],
    pub position_owner: solana_sdk::pubkey::Pubkey,
    pub pool_id: solana_sdk::pubkey::Pubkey,
    pub position_id: solana_sdk::pubkey::Pubkey,
    pub locked_nft_account: solana_sdk::pubkey::Pubkey,
    pub fee_nft_mint: solana_sdk::pubkey::Pubkey,
    pub recent_epoch: u64,
    pub padding: [u64; 8],
}
