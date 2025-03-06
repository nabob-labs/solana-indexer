use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x04")]
pub struct AdvanceNonceAccount;

pub struct AdvanceNonceAccountAccounts {
    pub nonce_account: solana_sdk::pubkey::Pubkey,
    pub recent_blockhashes_sysvar: solana_sdk::pubkey::Pubkey,
    pub nonce_authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for AdvanceNonceAccount {
    type ArrangedAccounts = AdvanceNonceAccountAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let nonce_account = accounts.get(0)?;
        let recent_blockhashes_sysvar = accounts.get(1)?;
        let nonce_authority = accounts.get(2)?;

        Some(AdvanceNonceAccountAccounts {
            nonce_account: nonce_account.pubkey,
            recent_blockhashes_sysvar: recent_blockhashes_sysvar.pubkey,
            nonce_authority: nonce_authority.pubkey,
        })
    }
}
