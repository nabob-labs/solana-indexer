use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x09")]
pub struct CloseAccount {}

pub struct CloseAccountAccounts {
    pub account: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CloseAccount {
    type ArrangedAccounts = CloseAccountAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let account = accounts.get(0)?;
        let destination = accounts.get(1)?;
        let owner = accounts.get(2)?;

        Some(CloseAccountAccounts {
            account: account.pubkey,
            destination: destination.pubkey,
            owner: owner.pubkey,
            remaining_accounts: accounts.get(3..).unwrap_or_default().to_vec(),
        })
    }
}
