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
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, destination, owner, remaining_accounts @ ..] = accounts else {
            return None;
        };

        Some(CloseAccountAccounts {
            account: account.pubkey,
            destination: destination.pubkey,
            owner: owner.pubkey,
            remaining_accounts: remaining_accounts.to_vec(),
        })
    }
}
