use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d304cf17590d7f22c")]
pub struct FeeParameterUpdateEvent {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub protocol_share: u16,
    pub base_factor: u16,
}
