use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum VaultConfigField {
    PerformanceFeeBps,
    ManagementFeeBps,
    MinDepositAmount,
    MinWithdrawAmount,
    MinInvestAmount,
    MinInvestDelaySlots,
    CrankFundFeePerReserve,
    PendingVaultAdmin,
    Name,
    LookupTable,
    Farm,
}
