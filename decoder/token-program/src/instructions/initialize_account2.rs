use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x10")]
pub struct InitializeAccount2 {
    pub owner: solana_sdk::pubkey::Pubkey,
}

pub struct InitializeAccount2Accounts {
    pub account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeAccount2 {
    type ArrangedAccounts = InitializeAccount2Accounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeAccount2Accounts {
            account: account.pubkey,
            mint: mint.pubkey,
        })
    }
}
