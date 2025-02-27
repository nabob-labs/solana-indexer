use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x01")]
pub struct Assign {
    pub onwer: solana_sdk::pubkey::Pubkey,
}

pub struct AssignAccounts {
    pub assigned_account: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Assign {
    type ArrangedAccounts = AssignAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let assigned_account = accounts.first()?;

        Some(AssignAccounts {
            assigned_account: assigned_account.pubkey,
        })
    }
}
