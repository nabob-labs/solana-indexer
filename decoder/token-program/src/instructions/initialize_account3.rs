use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x12")]
pub struct InitializeAccount3 {
    pub owner: solana_sdk::pubkey::Pubkey,
}

pub struct InitializeAccount3Accounts {
    pub account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeAccount3 {
    type ArrangedAccounts = InitializeAccount3Accounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let account = accounts.get(0)?;
        let mint = accounts.get(1)?;

        Some(InitializeAccount3Accounts {
            account: account.pubkey,
            mint: mint.pubkey,
        })
    }
}
