use super::super::types::*;
use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0b")]
pub struct CancelMultipleOrdersByIdWithFreeFunds {
    pub params: CancelMultipleOrdersByIdParams,
}

pub struct CancelMultipleOrdersByIdWithFreeFundsInstructionAccounts {
    pub phoenix_program: solana_pubkey::Pubkey,
    pub log_authority: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub trader: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CancelMultipleOrdersByIdWithFreeFunds {
    type ArrangedAccounts = CancelMultipleOrdersByIdWithFreeFundsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [phoenix_program, log_authority, market, trader, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CancelMultipleOrdersByIdWithFreeFundsInstructionAccounts {
            phoenix_program: phoenix_program.pubkey,
            log_authority: log_authority.pubkey,
            market: market.pubkey,
            trader: trader.pubkey,
        })
    }
}
