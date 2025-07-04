use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1dc0f1c9d946965af7")]
pub struct WithdrawEvent {
    pub dca_key: solana_pubkey::Pubkey,
    pub in_amount: u64,
    pub out_amount: u64,
    pub user_withdraw: bool,
}
