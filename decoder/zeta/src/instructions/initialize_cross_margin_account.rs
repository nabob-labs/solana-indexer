use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x1b1ae432d2d3cd5e")]
pub struct InitializeCrossMarginAccount {
    pub subaccount_index: u8,
}

pub struct InitializeCrossMarginAccountInstructionAccounts {
    pub cross_margin_account: solana_pubkey::Pubkey,
    pub cross_margin_account_manager: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub zeta_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeCrossMarginAccount {
    type ArrangedAccounts = InitializeCrossMarginAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [cross_margin_account, cross_margin_account_manager, authority, payer, zeta_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeCrossMarginAccountInstructionAccounts {
            cross_margin_account: cross_margin_account.pubkey,
            cross_margin_account_manager: cross_margin_account_manager.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            zeta_program: zeta_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
