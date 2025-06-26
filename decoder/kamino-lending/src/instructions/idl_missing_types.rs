use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x8250269950d4b6fd")]
pub struct IdlMissingTypes {
    pub reserve_farm_kind: ReserveFarmKind,
    pub asset_tier: AssetTier,
    pub fee_calculation: FeeCalculation,
    pub reserve_status: ReserveStatus,
    pub update_config_mode: UpdateConfigMode,
    pub update_lending_market_config_value: UpdateLendingMarketConfigValue,
    pub update_lending_market_config_mode: UpdateLendingMarketMode,
}

pub struct IdlMissingTypesInstructionAccounts {
    pub lending_market_owner: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for IdlMissingTypes {
    type ArrangedAccounts = IdlMissingTypesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lending_market_owner, lending_market, reserve, _remaining @ ..] = accounts else {
            return None;
        };

        Some(IdlMissingTypesInstructionAccounts {
            lending_market_owner: lending_market_owner.pubkey,
            lending_market: lending_market.pubkey,
            reserve: reserve.pubkey,
        })
    }
}
