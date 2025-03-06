use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x16")]
pub struct InitializeImmutableOwner {}

pub struct InitializeImmutableOwnerAccounts {
    pub account: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeImmutableOwner {
    type ArrangedAccounts = InitializeImmutableOwnerAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let account = accounts.get(0)?;

        Some(InitializeImmutableOwnerAccounts {
            account: account.pubkey,
        })
    }
}
