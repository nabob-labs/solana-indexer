use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0xcb12dc677891bb02")]
pub struct LastWithdraw {
    pub last_withdraw_timestamp: i64,
}
