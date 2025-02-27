use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x42506bba1bf2425f")]
pub struct UpdateUserAdvancedLp {
    pub sub_account_id: u16,
    pub advanced_lp: bool,
}

pub struct UpdateUserAdvancedLpInstructionAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateUserAdvancedLp {
    type ArrangedAccounts = UpdateUserAdvancedLpInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateUserAdvancedLpInstructionAccounts {
            user: user.pubkey,
            authority: authority.pubkey,
        })
    }
}
