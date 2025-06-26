use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug)]
#[indexer(discriminator = "0x74dbcce5f974ff96")]
pub struct TokenBadge {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
}
