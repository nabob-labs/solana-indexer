use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1ded6b098b164b04d5")]
pub struct IncreasePositionPreSwapEvent {
    pub position_request_key: solana_pubkey::Pubkey,
    pub transfer_amount: u64,
    pub collateral_custody_pre_swap_amount: u64,
}
