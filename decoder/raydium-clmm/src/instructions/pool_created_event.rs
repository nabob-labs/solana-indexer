use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x195e4b2f7063353f")]
pub struct PoolCreatedEvent {
    pub token_mint0: solana_pubkey::Pubkey,
    pub token_mint1: solana_pubkey::Pubkey,
    pub tick_spacing: u16,
    pub pool_state: solana_pubkey::Pubkey,
    pub sqrt_price_x64: u128,
    pub tick: i32,
    pub token_vault0: solana_pubkey::Pubkey,
    pub token_vault1: solana_pubkey::Pubkey,
}
