use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xc1d20df971951d54")]
pub struct GetAssetsUnderManagement2 {
    pub params: GetAssetsUnderManagement2Params,
}

pub struct GetAssetsUnderManagement2InstructionAccounts {
    pub perpetuals: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for GetAssetsUnderManagement2 {
    type ArrangedAccounts = GetAssetsUnderManagement2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [perpetuals, pool, _remaining @ ..] = accounts else {
            return None;
        };

        Some(GetAssetsUnderManagement2InstructionAccounts {
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
        })
    }
}
