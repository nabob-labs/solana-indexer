use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x883f0fbad398a8a4")]
pub struct InitObligationFarmsForReserve {
    pub mode: u8,
}

pub struct InitObligationFarmsForReserveInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub obligation: solana_sdk::pubkey::Pubkey,
    pub lending_market_authority: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub reserve_farm_state: solana_sdk::pubkey::Pubkey,
    pub obligation_farm: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub farms_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitObligationFarmsForReserve {
    type ArrangedAccounts = InitObligationFarmsForReserveInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, owner, obligation, lending_market_authority, reserve, reserve_farm_state, obligation_farm, lending_market, farms_program, rent, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitObligationFarmsForReserveInstructionAccounts {
            payer: payer.pubkey,
            owner: owner.pubkey,
            obligation: obligation.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            reserve: reserve.pubkey,
            reserve_farm_state: reserve_farm_state.pubkey,
            obligation_farm: obligation_farm.pubkey,
            lending_market: lending_market.pubkey,
            farms_program: farms_program.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
