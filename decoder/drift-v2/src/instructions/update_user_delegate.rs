use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x8bcd8d8d71245ebb")]
pub struct UpdateUserDelegate {
    pub sub_account_id: u16,
    pub delegate: solana_pubkey::Pubkey,
}

pub struct UpdateUserDelegateInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateUserDelegate {
    type ArrangedAccounts = UpdateUserDelegateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateUserDelegateInstructionAccounts {
            user: user.pubkey,
            authority: authority.pubkey,
        })
    }
}
