use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x07")]
pub struct AuthorizeNonceAccount(solana_sdk::pubkey::Pubkey);

pub struct AuthorizeNonceAccountAccounts {
    pub nonce_account: solana_sdk::pubkey::Pubkey,
    pub nonce_authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for AuthorizeNonceAccount {
    type ArrangedAccounts = AuthorizeNonceAccountAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let nonce_account = accounts.get(0)?;
        let nonce_authority = accounts.get(1)?;

        Some(AuthorizeNonceAccountAccounts {
            nonce_account: nonce_account.pubkey,
            nonce_authority: nonce_authority.pubkey,
        })
    }
}
