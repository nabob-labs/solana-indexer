use solana_indexer_core::{borsh, IndexerDeserialize};
use serde_big_array::BigArray;

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xd19d35d261b41f2d")]
pub struct UpdateLendingMarket {
    pub mode: u64,
    #[serde(with = "BigArray")]
    pub value: [u8; 72],
}

pub struct UpdateLendingMarketInstructionAccounts {
    pub lending_market_owner: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateLendingMarket {
    type ArrangedAccounts = UpdateLendingMarketInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let lending_market_owner = accounts.get(0)?;
        let lending_market = accounts.get(1)?;

        Some(UpdateLendingMarketInstructionAccounts {
            lending_market_owner: lending_market_owner.pubkey,
            lending_market: lending_market.pubkey,
        })
    }
}
