use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x4cc95012c75cf669")]
pub struct OperatorSetPoolConfig {
    pub params: OperatorSetPoolConfigParams,
}

pub struct OperatorSetPoolConfigInstructionAccounts {
    pub operator: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for OperatorSetPoolConfig {
    type ArrangedAccounts = OperatorSetPoolConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [operator, pool, _remaining @ ..] = accounts else {
            return None;
        };

        Some(OperatorSetPoolConfigInstructionAccounts {
            operator: operator.pubkey,
            pool: pool.pubkey,
        })
    }
}
