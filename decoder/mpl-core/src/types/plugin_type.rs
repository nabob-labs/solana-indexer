

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum PluginType {
    Royalties,
    FreezeDelegate,
    BurnDelegate,
    TransferDelegate,
    UpdateDelegate,
    PermanentFreezeDelegate,
    Attributes,
    PermanentTransferDelegate,
    PermanentBurnDelegate,
    Edition,
    MasterEdition,
    AddBlocker,
    ImmutableMetadata,
    VerifiedCreators,
    Autograph,
}


