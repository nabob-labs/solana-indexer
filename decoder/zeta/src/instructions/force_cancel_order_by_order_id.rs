use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xb6eb30b3f885d2f0")]
pub struct ForceCancelOrderByOrderId {
    pub side: Side,
    pub order_id: u128,
    pub asset: Asset,
}

pub struct ForceCancelOrderByOrderIdInstructionAccounts {
    pub zeta_group: solana_pubkey::Pubkey,
    pub greeks: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub oracle_backup_feed: solana_pubkey::Pubkey,
    pub oracle_backup_program: solana_pubkey::Pubkey,
    pub cancel_accounts: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ForceCancelOrderByOrderId {
    type ArrangedAccounts = ForceCancelOrderByOrderIdInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [zeta_group, greeks, oracle, oracle_backup_feed, oracle_backup_program, cancel_accounts, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ForceCancelOrderByOrderIdInstructionAccounts {
            zeta_group: zeta_group.pubkey,
            greeks: greeks.pubkey,
            oracle: oracle.pubkey,
            oracle_backup_feed: oracle_backup_feed.pubkey,
            oracle_backup_program: oracle_backup_program.pubkey,
            cancel_accounts: cancel_accounts.pubkey,
        })
    }
}
