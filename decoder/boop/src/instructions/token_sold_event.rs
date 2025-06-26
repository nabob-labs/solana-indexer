use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1dccefb64df1334d42")]
pub struct TokenSoldEvent {
    pub mint: solana_pubkey::Pubkey,
    pub amount_in: u64,
    pub amount_out: u64,
    pub swap_fee: u64,
    pub seller: solana_pubkey::Pubkey,
    pub recipient: solana_pubkey::Pubkey,
}
