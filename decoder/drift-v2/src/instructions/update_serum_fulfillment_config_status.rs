use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xab6df0fb5f019559")]
pub struct UpdateSerumFulfillmentConfigStatus {
    pub status: SpotFulfillmentConfigStatus,
}

pub struct UpdateSerumFulfillmentConfigStatusInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub serum_fulfillment_config: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateSerumFulfillmentConfigStatus {
    type ArrangedAccounts = UpdateSerumFulfillmentConfigStatusInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, serum_fulfillment_config, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateSerumFulfillmentConfigStatusInstructionAccounts {
            state: state.pubkey,
            serum_fulfillment_config: serum_fulfillment_config.pubkey,
            admin: admin.pubkey,
        })
    }
}
