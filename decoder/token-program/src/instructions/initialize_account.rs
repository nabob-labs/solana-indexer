use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x01")]
pub struct InitializeAccount {
    pub amount: u64,
}

pub struct InitializeAccountAccounts {
    pub account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeAccount {
    type ArrangedAccounts = InitializeAccountAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let account = accounts.get(0)?;
        let mint = accounts.get(1)?;
        let owner = accounts.get(2)?;

        Some(InitializeAccountAccounts {
            account: account.pubkey,
            mint: mint.pubkey,
            owner: owner.pubkey,
        })
    }
}
