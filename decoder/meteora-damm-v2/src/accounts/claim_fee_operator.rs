use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize)]
#[indexer(discriminator = "0xa630865622c8bc96")]
pub struct ClaimFeeOperator {
    pub operator: solana_pubkey::Pubkey,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u8; 128],
}
