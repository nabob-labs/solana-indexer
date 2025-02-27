use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xcbc4bb3c0daabe45")]
pub struct CloseCrossMarginAccount {
    pub subaccount_index: u8,
}

pub struct CloseCrossMarginAccountInstructionAccounts {
    pub cross_margin_account: solana_sdk::pubkey::Pubkey,
    pub cross_margin_account_manager: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CloseCrossMarginAccount {
    type ArrangedAccounts = CloseCrossMarginAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [cross_margin_account, cross_margin_account_manager, authority, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CloseCrossMarginAccountInstructionAccounts {
            cross_margin_account: cross_margin_account.pubkey,
            cross_margin_account_manager: cross_margin_account_manager.pubkey,
            authority: authority.pubkey,
        })
    }
}
