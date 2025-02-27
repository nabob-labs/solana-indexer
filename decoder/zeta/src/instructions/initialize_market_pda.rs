use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x057864bae186080d")]
pub struct InitializeMarketPda {
    pub asset: Asset,
}

pub struct InitializeMarketPdaInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub market_indexes: solana_sdk::pubkey::Pubkey,
    pub pricing: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeMarketPda {
    type ArrangedAccounts = InitializeMarketPdaInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, market_indexes, pricing, admin, market, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeMarketPdaInstructionAccounts {
            state: state.pubkey,
            market_indexes: market_indexes.pubkey,
            pricing: pricing.pubkey,
            admin: admin.pubkey,
            market: market.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
