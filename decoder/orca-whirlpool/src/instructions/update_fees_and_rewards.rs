use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x9ae6fa0decd14bdf")]
pub struct UpdateFeesAndRewards {}

pub struct UpdateFeesAndRewardsInstructionAccounts {
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub tick_array_lower: solana_sdk::pubkey::Pubkey,
    pub tick_array_upper: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateFeesAndRewards {
    type ArrangedAccounts = UpdateFeesAndRewardsInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let whirlpool = accounts.get(0)?;
        let position = accounts.get(1)?;
        let tick_array_lower = accounts.get(2)?;
        let tick_array_upper = accounts.get(3)?;

        Some(UpdateFeesAndRewardsInstructionAccounts {
            whirlpool: whirlpool.pubkey,
            position: position.pubkey,
            tick_array_lower: tick_array_lower.pubkey,
            tick_array_upper: tick_array_upper.pubkey,
        })
    }
}
