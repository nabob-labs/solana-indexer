use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xdb56496a38da806d")]
pub struct UpdateUserPoolId {
    pub sub_account_id: u16,
    pub pool_id: u8,
}

pub struct UpdateUserPoolIdInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateUserPoolId {
    type ArrangedAccounts = UpdateUserPoolIdInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateUserPoolIdInstructionAccounts {
            user: user.pubkey,
            authority: authority.pubkey,
        })
    }
}
