use alloc::string::String;
use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize)]
#[indexer(discriminator = "0x4444821310d1629c")]
pub struct PartnerMetadata {
    pub fee_claimer: solana_pubkey::Pubkey,
    pub padding: [u128; 6],
    pub name: String,
    pub website: String,
    pub logo: String,
}
