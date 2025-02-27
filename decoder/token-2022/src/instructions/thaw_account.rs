use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0b")]
pub struct ThawAccount {}

pub struct ThawAccountInstructionAccounts {
    pub account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ThawAccount {
    type ArrangedAccounts = ThawAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, mint, owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ThawAccountInstructionAccounts {
            account: account.pubkey,
            mint: mint.pubkey,
            owner: owner.pubkey,
        })
    }
}
