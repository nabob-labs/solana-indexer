use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1dc87f06370d200896")]
pub struct EvtPartnerMetadataEvent {
    pub partner_metadata: solana_pubkey::Pubkey,
    pub fee_claimer: solana_pubkey::Pubkey,
}
