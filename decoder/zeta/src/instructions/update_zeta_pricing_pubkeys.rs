use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xa9dd17f8db7a8e9e")]
pub struct UpdateZetaPricingPubkeys {
    pub args: UpdateZetaPricingPubkeysArgs,
}

pub struct UpdateZetaPricingPubkeysInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateZetaPricingPubkeys {
    type ArrangedAccounts = UpdateZetaPricingPubkeysInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, pricing, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateZetaPricingPubkeysInstructionAccounts {
            state: state.pubkey,
            pricing: pricing.pubkey,
            admin: admin.pubkey,
        })
    }
}
