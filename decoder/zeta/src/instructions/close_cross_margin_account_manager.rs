use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe8b6b689565876fc")]
pub struct CloseCrossMarginAccountManager {}

pub struct CloseCrossMarginAccountManagerInstructionAccounts {
    pub cross_margin_account_manager: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CloseCrossMarginAccountManager {
    type ArrangedAccounts = CloseCrossMarginAccountManagerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [cross_margin_account_manager, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CloseCrossMarginAccountManagerInstructionAccounts {
            cross_margin_account_manager: cross_margin_account_manager.pubkey,
            authority: authority.pubkey,
        })
    }
}
