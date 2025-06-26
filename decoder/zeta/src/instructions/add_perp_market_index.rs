use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x7a280e40a912e788")]
pub struct AddPerpMarketIndex {
    pub asset: Asset,
}

pub struct AddPerpMarketIndexInstructionAccounts {
    pub market_indexes: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for AddPerpMarketIndex {
    type ArrangedAccounts = AddPerpMarketIndexInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [market_indexes, pricing, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AddPerpMarketIndexInstructionAccounts {
            market_indexes: market_indexes.pubkey,
            pricing: pricing.pubkey,
        })
    }
}
