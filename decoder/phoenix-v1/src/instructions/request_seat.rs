use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0e")]
pub struct RequestSeat {}

pub struct RequestSeatInstructionAccounts {
    pub phoenix_program: solana_pubkey::Pubkey,
    pub log_authority: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub seat: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for RequestSeat {
    type ArrangedAccounts = RequestSeatInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [phoenix_program, log_authority, market, payer, seat, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RequestSeatInstructionAccounts {
            phoenix_program: phoenix_program.pubkey,
            log_authority: log_authority.pubkey,
            market: market.pubkey,
            payer: payer.pubkey,
            seat: seat.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
