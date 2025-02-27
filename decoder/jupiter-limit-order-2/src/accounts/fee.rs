use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x183796faa81b65b2")]
pub struct Fee {
    pub normal_fee_bps: u16,
    pub stable_fee_bps: u16,
}
