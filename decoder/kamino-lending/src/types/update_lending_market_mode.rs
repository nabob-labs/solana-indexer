use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum UpdateLendingMarketMode {
    UpdateOwner,
    UpdateEmergencyMode,
    UpdateLiquidationCloseFactor,
    UpdateLiquidationMaxValue,
    UpdateGlobalUnhealthyBorrow,
    UpdateGlobalAllowedBorrow,
    UpdateRiskCouncil,
    UpdateMinFullLiquidationThreshold,
    UpdateInsolvencyRiskLtv,
    UpdateElevationGroup,
    UpdateReferralFeeBps,
    DeprecatedUpdateMultiplierPoints,
    UpdatePriceRefreshTriggerToMaxAgePct,
    UpdateAutodeleverageEnabled,
    UpdateBorrowingDisabled,
    UpdateMinNetValueObligationPostAction,
    UpdateMinValueSkipPriorityLiqCheck,
    UpdatePaddingFields,
    UpdateName,
}
