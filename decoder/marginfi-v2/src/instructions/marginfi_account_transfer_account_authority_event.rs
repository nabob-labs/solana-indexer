use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d703d8c84fb5c5aca")]
pub struct MarginfiAccountTransferAccountAuthorityEvent {
    pub header: AccountEventHeader,
    pub old_account_authority: solana_pubkey::Pubkey,
    pub new_account_authority: solana_pubkey::Pubkey,
}
