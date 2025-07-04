use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x6eca0e2a5f495a5f")]
pub struct InsuranceFundStake {
    pub authority: solana_pubkey::Pubkey,
    pub if_shares: u128,
    pub last_withdraw_request_shares: u128,
    pub if_base: u128,
    pub last_valid_ts: i64,
    pub last_withdraw_request_value: u64,
    pub last_withdraw_request_ts: i64,
    pub cost_basis: i64,
    pub market_index: u16,
    pub padding: [u8; 14],
}
