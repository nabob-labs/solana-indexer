use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xede119edc12d4d61")]
pub struct UpdateStateMaxInitializeUserFee {
    pub max_initialize_user_fee: u16,
}

pub struct UpdateStateMaxInitializeUserFeeInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateStateMaxInitializeUserFee {
    type ArrangedAccounts = UpdateStateMaxInitializeUserFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateStateMaxInitializeUserFeeInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
