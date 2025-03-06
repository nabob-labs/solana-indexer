use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x07")]
pub struct MintTo {
    pub amount: u64,
}

pub struct MintToAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub account: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for MintTo {
    type ArrangedAccounts = MintToAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.get(0)?;
        let account = accounts.get(1)?;
        let authority = accounts.get(2)?;

        Some(MintToAccounts {
            mint: mint.pubkey,
            account: account.pubkey,
            authority: authority.pubkey,
            remaining_accounts: accounts.get(3..).unwrap_or_default().to_vec(),
        })
    }
}
