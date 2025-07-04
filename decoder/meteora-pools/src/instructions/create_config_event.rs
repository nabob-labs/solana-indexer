use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1dc7980a1327279d68")]
pub struct CreateConfigEvent {
    pub trade_fee_numerator: u64,
    pub protocol_trade_fee_numerator: u64,
    pub config: solana_pubkey::Pubkey,
}
