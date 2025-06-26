use super::super::types::*;
use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x03")]
pub struct PlaceLimitOrderWithFreeFunds {
    pub order_packet: OrderPacket,
}

pub struct PlaceLimitOrderWithFreeFundsInstructionAccounts {
    pub phoenix_program: solana_pubkey::Pubkey,
    pub log_authority: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub trader: solana_pubkey::Pubkey,
    pub seat: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for PlaceLimitOrderWithFreeFunds {
    type ArrangedAccounts = PlaceLimitOrderWithFreeFundsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [phoenix_program, log_authority, market, trader, seat, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(PlaceLimitOrderWithFreeFundsInstructionAccounts {
            phoenix_program: phoenix_program.pubkey,
            log_authority: log_authority.pubkey,
            market: market.pubkey,
            trader: trader.pubkey,
            seat: seat.pubkey,
        })
    }
}
