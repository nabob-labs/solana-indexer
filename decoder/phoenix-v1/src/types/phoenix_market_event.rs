use super::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum PhoenixMarketEvent {
    Uninitialized,
    Header(AuditLogHeader),
    Fill(FillEvent),
    Place(PlaceEvent),
    Reduce(ReduceEvent),
    Evict(EvictEvent),
    FillSummary(FillSummaryEvent),
    Fee(FeeEvent),
    TimeInForce(TimeInForceEvent),
    ExpiredOrder(ExpiredOrderEvent),
}
