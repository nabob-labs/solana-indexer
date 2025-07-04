use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1decba71db2a3395f9")]
pub struct NewUserRecordEvent {
    pub ts: i64,
    pub user_authority: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub sub_account_id: u16,
    pub name: [u8; 32],
    pub referrer: solana_pubkey::Pubkey,
}
