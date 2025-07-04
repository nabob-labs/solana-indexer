use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x1752e1dedb7ae6fb")]
pub struct ApplyPerpFunding {
    pub asset: Asset,
}

pub struct ApplyPerpFundingInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ApplyPerpFunding {
    type ArrangedAccounts = ApplyPerpFundingInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, pricing, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ApplyPerpFundingInstructionAccounts {
            state: state.pubkey,
            pricing: pricing.pubkey,
        })
    }
}
