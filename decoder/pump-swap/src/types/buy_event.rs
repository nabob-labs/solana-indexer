use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct BuyEvent {
    pub timestamp: i64,
    pub base_amount_out: u64,
    pub max_quote_amount_in: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub quote_amount_in: u64,
    pub lp_fee_basis_points: u64,
    pub lp_fee: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee: u64,
    pub quote_amount_in_with_lp_fee: u64,
    pub user_quote_amount_in: u64,
    pub pool: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub user_base_token_account: solana_pubkey::Pubkey,
    pub user_quote_token_account: solana_pubkey::Pubkey,
    pub protocol_fee_recipient: solana_pubkey::Pubkey,
    pub protocol_fee_recipient_token_account: solana_pubkey::Pubkey,
    pub coin_creator: solana_pubkey::Pubkey,
    pub coin_creator_fee_basis_points: u64,
    pub coin_creator_fee: u64,
}
