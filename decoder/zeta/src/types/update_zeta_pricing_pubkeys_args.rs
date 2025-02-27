use super::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateZetaPricingPubkeysArgs {
    pub asset: Asset,
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub perp_sync_queue: solana_sdk::pubkey::Pubkey,
    pub zeta_group_key: solana_sdk::pubkey::Pubkey,
    pub reset_oracle: bool,
}
