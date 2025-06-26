use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AccountEventHeader {
    pub signer: Option<solana_pubkey::Pubkey>,
    pub marginfi_account: solana_pubkey::Pubkey,
    pub marginfi_account_authority: solana_pubkey::Pubkey,
    pub marginfi_group: solana_pubkey::Pubkey,
}
