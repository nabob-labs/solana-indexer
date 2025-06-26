use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe445a52e51cb9a1d68e9ed7ac7bf7955")]
pub struct EvtFundRewardEvent {
    pub pool: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub mint_reward: solana_pubkey::Pubkey,
    pub reward_index: u8,
    pub amount: u64,
    pub transfer_fee_excluded_amount_in: u64,
}
