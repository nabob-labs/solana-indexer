use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d80977b6a1166718e")]
pub struct CompositionFeeEvent {
    pub from: solana_pubkey::Pubkey,
    pub bin_id: i16,
    pub token_x_fee_amount: u64,
    pub token_y_fee_amount: u64,
    pub protocol_token_x_fee_amount: u64,
    pub protocol_token_y_fee_amount: u64,
}
