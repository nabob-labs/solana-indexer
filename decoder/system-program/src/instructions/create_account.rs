use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x00")]
pub struct CreateAccount {
    pub lamports: u64,
    pub space: u64,
    pub onwer: solana_sdk::pubkey::Pubkey,
}

pub struct CreateAccountAccounts {
    pub funding_account: solana_sdk::pubkey::Pubkey,
    pub new_account: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CreateAccount {
    type ArrangedAccounts = CreateAccountAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let funding_account = accounts.get(0)?;
        let new_account = accounts.get(1)?;

        Some(CreateAccountAccounts {
            funding_account: funding_account.pubkey,
            new_account: new_account.pubkey,
        })
    }
}
