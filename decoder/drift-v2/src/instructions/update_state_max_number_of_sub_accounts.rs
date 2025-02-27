use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x9b7bd602dda6cc55")]
pub struct UpdateStateMaxNumberOfSubAccounts {
    pub max_number_of_sub_accounts: u16,
}

pub struct UpdateStateMaxNumberOfSubAccountsInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateStateMaxNumberOfSubAccounts {
    type ArrangedAccounts = UpdateStateMaxNumberOfSubAccountsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateStateMaxNumberOfSubAccountsInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
