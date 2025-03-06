use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0a")]
pub struct FreezeAccount {}

pub struct FreezeAccountAccounts {
    pub account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for FreezeAccount {
    type ArrangedAccounts = FreezeAccountAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let account = accounts.get(0)?;
        let mint = accounts.get(1)?;
        let authority = accounts.get(2)?;

        Some(FreezeAccountAccounts {
            account: account.pubkey,
            mint: mint.pubkey,
            authority: authority.pubkey,
            remaining_accounts: accounts.get(3..).unwrap_or_default().to_vec(),
        })
    }
}
