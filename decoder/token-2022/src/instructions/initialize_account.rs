use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x01")]
pub struct InitializeAccount {}

pub struct InitializeAccountInstructionAccounts {
    pub account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeAccount {
    type ArrangedAccounts = InitializeAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, mint, owner, rent, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeAccountInstructionAccounts {
            account: account.pubkey,
            mint: mint.pubkey,
            owner: owner.pubkey,
            rent: rent.pubkey,
        })
    }
}
