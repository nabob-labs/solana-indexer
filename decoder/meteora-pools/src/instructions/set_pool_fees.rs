use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x662c9e36cd257e4e")]
pub struct SetPoolFees {
    pub fees: PoolFees,
    pub new_partner_fee_numerator: u64,
}

pub struct SetPoolFeesInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub fee_operator: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetPoolFees {
    type ArrangedAccounts = SetPoolFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, fee_operator, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetPoolFeesInstructionAccounts {
            pool: pool.pubkey,
            fee_operator: fee_operator.pubkey,
        })
    }
}
