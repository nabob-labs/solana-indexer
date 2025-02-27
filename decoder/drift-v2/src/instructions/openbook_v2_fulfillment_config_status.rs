use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x19ad13bd04d340ee")]
pub struct OpenbookV2FulfillmentConfigStatus {
    pub status: SpotFulfillmentConfigStatus,
}

pub struct OpenbookV2FulfillmentConfigStatusInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub openbook_v2_fulfillment_config: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for OpenbookV2FulfillmentConfigStatus {
    type ArrangedAccounts = OpenbookV2FulfillmentConfigStatusInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, openbook_v2_fulfillment_config, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(OpenbookV2FulfillmentConfigStatusInstructionAccounts {
            state: state.pubkey,
            openbook_v2_fulfillment_config: openbook_v2_fulfillment_config.pubkey,
            admin: admin.pubkey,
        })
    }
}
