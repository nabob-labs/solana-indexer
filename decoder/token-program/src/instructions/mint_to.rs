use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x07")]
pub struct MintTo {
    pub amount: u64,
}

pub struct MintToAccounts {
    pub mint: solana_pubkey::Pubkey,
    pub account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for MintTo {
    type ArrangedAccounts = MintToAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, account, authority, remaining_accounts @ ..] = accounts else {
            return None;
        };

        Some(MintToAccounts {
            mint: mint.pubkey,
            account: account.pubkey,
            authority: authority.pubkey,
            remaining_accounts: remaining_accounts.to_vec(),
        })
    }
}
