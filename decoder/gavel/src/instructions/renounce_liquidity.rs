use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x03")]
pub struct RenounceLiquidity {
    pub params: RenounceLiquidityIxParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RenounceLiquidityInstructionAccounts {
    pub plasma_program: solana_pubkey::Pubkey,
    pub log_authority: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub trader: solana_pubkey::Pubkey,
    pub lp_position: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for RenounceLiquidity {
    type ArrangedAccounts = RenounceLiquidityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [plasma_program, log_authority, pool, trader, lp_position, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(RenounceLiquidityInstructionAccounts {
            plasma_program: plasma_program.pubkey,
            log_authority: log_authority.pubkey,
            pool: pool.pubkey,
            trader: trader.pubkey,
            lp_position: lp_position.pubkey,
        })
    }
}
