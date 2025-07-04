use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d79a3cdc939da753c")]
pub struct LpChangeEvent {
    pub pool_id: solana_pubkey::Pubkey,
    pub lp_amount_before: u64,
    pub token0_vault_before: u64,
    pub token1_vault_before: u64,
    pub token0_amount: u64,
    pub token1_amount: u64,
    pub token0_transfer_fee: u64,
    pub token1_transfer_fee: u64,
    pub change_type: u8,
}
