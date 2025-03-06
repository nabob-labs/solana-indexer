
use super::*;

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum ExternalPluginAdapter {
    LifecycleHook
                (
                    LifecycleHook,
                )
    ,
    Oracle
                (
                    Oracle,
                )
    ,
    AppData
                (
                    AppData,
                )
    ,
    LinkedLifecycleHook
                (
                    LinkedLifecycleHook,
                )
    ,
    LinkedAppData
                (
                    LinkedAppData,
                )
    ,
    DataSection
                (
                    DataSection,
                )
    ,
}


