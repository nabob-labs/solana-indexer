use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x9d356b68b8bd64dc")]
pub struct MigrateToCrossMarginAccount {}

pub struct MigrateToCrossMarginAccountInstructionAccounts {
    pub cross_margin_account: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for MigrateToCrossMarginAccount {
    type ArrangedAccounts = MigrateToCrossMarginAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [cross_margin_account, pricing, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(MigrateToCrossMarginAccountInstructionAccounts {
            cross_margin_account: cross_margin_account.pubkey,
            pricing: pricing.pubkey,
            authority: authority.pubkey,
        })
    }
}
