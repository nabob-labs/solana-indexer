use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum Asset {
    SOL,
    BTC,
    ETH,
    APT,
    ARB,
    BERA,
    PYTH,
    TIA,
    JTO,
    ONEMBONK,
    SEI,
    JUP,
    DYM,
    STRK,
    WIF,
    RNDR,
    TNSR,
    POPCAT,
    EIGEN,
    DBR,
    GOAT,
    DRIFT,
    PNUT,
    PENGU,
    TRUMP,
    UNDEFINED,
}
