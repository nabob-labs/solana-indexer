use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe3ea9df6804ae936")]
pub struct UpdateTakeTriggerOrderFeePercentage {
    pub new_take_trigger_order_fee_percentage: u64,
}

pub struct UpdateTakeTriggerOrderFeePercentageInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateTakeTriggerOrderFeePercentage {
    type ArrangedAccounts = UpdateTakeTriggerOrderFeePercentageInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateTakeTriggerOrderFeePercentageInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
        })
    }
}
