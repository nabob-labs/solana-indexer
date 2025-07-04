use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x58df84a1d0588e2a")]
pub struct InitializeFuelOverflow {}

pub struct InitializeFuelOverflowInstructionAccounts {
    pub fuel_overflow: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeFuelOverflow {
    type ArrangedAccounts = InitializeFuelOverflowInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [fuel_overflow, user_stats, authority, payer, rent, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeFuelOverflowInstructionAccounts {
            fuel_overflow: fuel_overflow.pubkey,
            user_stats: user_stats.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
