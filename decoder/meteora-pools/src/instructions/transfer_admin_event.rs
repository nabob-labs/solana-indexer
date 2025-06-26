use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1de4a983f43d3841fe")]
pub struct TransferAdminEvent {
    pub admin: solana_pubkey::Pubkey,
    pub new_admin: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
}
