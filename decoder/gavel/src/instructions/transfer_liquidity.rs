use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x09")]
pub struct TransferLiquidity {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TransferLiquidityInstructionAccounts {
    pub plasma_program: solana_pubkey::Pubkey,
    pub log_authority: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub trader: solana_pubkey::Pubkey,
    pub src_lp_position: solana_pubkey::Pubkey,
    pub dst_lp_position: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for TransferLiquidity {
    type ArrangedAccounts = TransferLiquidityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [plasma_program, log_authority, pool, trader, src_lp_position, dst_lp_position, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(TransferLiquidityInstructionAccounts {
            plasma_program: plasma_program.pubkey,
            log_authority: log_authority.pubkey,
            pool: pool.pubkey,
            trader: trader.pubkey,
            src_lp_position: src_lp_position.pubkey,
            dst_lp_position: dst_lp_position.pubkey,
        })
    }
}
