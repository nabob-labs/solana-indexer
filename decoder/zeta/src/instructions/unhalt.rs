use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xf98c1bd58082cf71")]
pub struct Unhalt {
    pub asset: Asset,
}

pub struct UnhaltInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Unhalt {
    type ArrangedAccounts = UnhaltInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, pricing, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UnhaltInstructionAccounts {
            state: state.pubkey,
            pricing: pricing.pubkey,
            admin: admin.pubkey,
        })
    }
}
