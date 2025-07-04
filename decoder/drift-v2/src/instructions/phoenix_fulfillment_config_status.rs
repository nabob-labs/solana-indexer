use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x601f71200ccb079a")]
pub struct PhoenixFulfillmentConfigStatus {
    pub status: SpotFulfillmentConfigStatus,
}

pub struct PhoenixFulfillmentConfigStatusInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub phoenix_fulfillment_config: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for PhoenixFulfillmentConfigStatus {
    type ArrangedAccounts = PhoenixFulfillmentConfigStatusInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, phoenix_fulfillment_config, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(PhoenixFulfillmentConfigStatusInstructionAccounts {
            state: state.pubkey,
            phoenix_fulfillment_config: phoenix_fulfillment_config.pubkey,
            admin: admin.pubkey,
        })
    }
}
