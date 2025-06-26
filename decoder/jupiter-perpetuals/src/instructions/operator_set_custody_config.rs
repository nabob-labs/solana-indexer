use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xa6895ccc91e018da")]
pub struct OperatorSetCustodyConfig {
    pub params: OperatorSetCustodyConfigParams,
}

pub struct OperatorSetCustodyConfigInstructionAccounts {
    pub operator: solana_pubkey::Pubkey,
    pub custody: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for OperatorSetCustodyConfig {
    type ArrangedAccounts = OperatorSetCustodyConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [operator, custody, _remaining @ ..] = accounts else {
            return None;
        };

        Some(OperatorSetCustodyConfigInstructionAccounts {
            operator: operator.pubkey,
            custody: custody.pubkey,
        })
    }
}
