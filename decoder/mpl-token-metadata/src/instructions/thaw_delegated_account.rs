use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x1b")]
pub struct ThawDelegatedAccount {}

pub struct ThawDelegatedAccountInstructionAccounts {
    pub delegate: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ThawDelegatedAccount {
    type ArrangedAccounts = ThawDelegatedAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [delegate, token_account, edition, mint, token_program, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(ThawDelegatedAccountInstructionAccounts {
            delegate: delegate.pubkey,
            token_account: token_account.pubkey,
            edition: edition.pubkey,
            mint: mint.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
