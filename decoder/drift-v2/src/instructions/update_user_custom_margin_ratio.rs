use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x15dd8cbb20810b7b")]
pub struct UpdateUserCustomMarginRatio {
    pub sub_account_id: u16,
    pub margin_ratio: u32,
}

pub struct UpdateUserCustomMarginRatioInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateUserCustomMarginRatio {
    type ArrangedAccounts = UpdateUserCustomMarginRatioInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateUserCustomMarginRatioInstructionAccounts {
            user: user.pubkey,
            authority: authority.pubkey,
        })
    }
}
