use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xc78fad93cacc40cc")]
pub struct InitializeMarketTifEpochCycle {
    pub epoch_length: u16,
}

pub struct InitializeMarketTifEpochCycleInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub serum_authority: solana_pubkey::Pubkey,
    pub dex_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeMarketTifEpochCycle {
    type ArrangedAccounts = InitializeMarketTifEpochCycleInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, market, serum_authority, dex_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeMarketTifEpochCycleInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
            market: market.pubkey,
            serum_authority: serum_authority.pubkey,
            dex_program: dex_program.pubkey,
        })
    }
}
