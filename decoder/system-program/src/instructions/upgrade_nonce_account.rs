use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0C")]
pub struct UpgradeNonceAccount;

pub struct UpgradeNonceAccountAccounts {
    pub nonce_account: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpgradeNonceAccount {
    type ArrangedAccounts = UpgradeNonceAccountAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let nonce_account = accounts.get(0)?;

        Some(UpgradeNonceAccountAccounts {
            nonce_account: nonce_account.pubkey,
        })
    }
}
