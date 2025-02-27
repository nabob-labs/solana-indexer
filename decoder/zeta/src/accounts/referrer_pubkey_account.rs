use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x1d37607f524897c5")]
pub struct ReferrerPubkeyAccount {
    pub referrer_id: [u8; 6],
}
