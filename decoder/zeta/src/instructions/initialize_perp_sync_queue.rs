use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0a379ae081aea108")]
pub struct InitializePerpSyncQueue {
    pub nonce: u8,
    pub asset: Asset,
}

pub struct InitializePerpSyncQueueInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub zeta_program: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub perp_sync_queue: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializePerpSyncQueue {
    type ArrangedAccounts = InitializePerpSyncQueueInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, zeta_program, state, perp_sync_queue, pricing, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializePerpSyncQueueInstructionAccounts {
            admin: admin.pubkey,
            zeta_program: zeta_program.pubkey,
            state: state.pubkey,
            perp_sync_queue: perp_sync_queue.pubkey,
            pricing: pricing.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
