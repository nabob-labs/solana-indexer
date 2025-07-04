use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xc7472a439013566d")]
pub struct UpdateUserReduceOnly {
    pub sub_account_id: u16,
    pub reduce_only: bool,
}

pub struct UpdateUserReduceOnlyInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateUserReduceOnly {
    type ArrangedAccounts = UpdateUserReduceOnlyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateUserReduceOnlyInstructionAccounts {
            user: user.pubkey,
            authority: authority.pubkey,
        })
    }
}
