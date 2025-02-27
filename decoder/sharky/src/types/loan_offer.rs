use {
    super::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LoanOffer {
    pub lender_wallet: solana_sdk::pubkey::Pubkey,
    pub terms_spec: LoanTermsSpec,
    pub offer_time: i64,
}
