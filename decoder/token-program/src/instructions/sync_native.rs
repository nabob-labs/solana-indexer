use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x11")]
pub struct SyncNative {}

pub struct SyncNativeAccounts {
    pub account: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SyncNative {
    type ArrangedAccounts = SyncNativeAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let account = accounts.first()?;

        Some(SyncNativeAccounts {
            account: account.pubkey,
        })
    }
}
