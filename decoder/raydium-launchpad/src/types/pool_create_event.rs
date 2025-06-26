use super::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PoolCreateEvent {
    pub pool_state: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub base_mint_param: MintParams,
    pub curve_param: CurveParams,
    pub vesting_param: VestingParams,
}
