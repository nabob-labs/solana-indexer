use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d9fccc08a4491e094")]
pub struct ConfigLpEvent {
    pub state: solana_pubkey::Pubkey,
    pub min_fee_change: Option<FeeValueChange>,
    pub max_fee_change: Option<FeeValueChange>,
    pub liquidity_target_change: Option<U64ValueChange>,
    pub treasury_cut_change: Option<FeeValueChange>,
}
