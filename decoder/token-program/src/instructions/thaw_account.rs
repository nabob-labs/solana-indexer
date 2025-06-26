use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0b")]
pub struct ThawAccount {}

pub struct ThawAccountAccounts {
    pub account: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ThawAccount {
    type ArrangedAccounts = ThawAccountAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, mint, authority, remaining_accounts @ ..] = accounts else {
            return None;
        };

        Some(ThawAccountAccounts {
            account: account.pubkey,
            mint: mint.pubkey,
            authority: authority.pubkey,
            remaining_accounts: remaining_accounts.to_vec(),
        })
    }
}
