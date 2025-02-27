use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x13")]
pub struct InitializeMultisig2 {
    pub m: u8,
}

pub struct InitializeMultisig2Accounts {
    pub account: solana_sdk::pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeMultisig2 {
    type ArrangedAccounts = InitializeMultisig2Accounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let account = accounts.first()?;

        Some(InitializeMultisig2Accounts {
            account: account.pubkey,
            remaining_accounts: accounts.get(1..).unwrap_or_default().to_vec(),
        })
    }
}
