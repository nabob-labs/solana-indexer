use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xb72dfb6d866cbff3")]
pub struct MigrateToNewCrossMarginAccount {}

pub struct MigrateToNewCrossMarginAccountInstructionAccounts {
    pub new_cross_margin_account: solana_pubkey::Pubkey,
    pub old_cross_margin_account: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for MigrateToNewCrossMarginAccount {
    type ArrangedAccounts = MigrateToNewCrossMarginAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [new_cross_margin_account, old_cross_margin_account, pricing, authority, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(MigrateToNewCrossMarginAccountInstructionAccounts {
            new_cross_margin_account: new_cross_margin_account.pubkey,
            old_cross_margin_account: old_cross_margin_account.pubkey,
            pricing: pricing.pubkey,
            authority: authority.pubkey,
        })
    }
}
