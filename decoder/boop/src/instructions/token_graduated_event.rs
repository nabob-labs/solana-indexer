use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d49746f1a5cd9928d")]
pub struct TokenGraduatedEvent {
    pub mint: solana_pubkey::Pubkey,
    pub sol_for_liquidity: u64,
    pub graduation_fee: u64,
    pub token_for_distributor: u64,
}
