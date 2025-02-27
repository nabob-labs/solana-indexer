use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x76e00a3ec4e6b859")]
pub struct UpdateLendingMarketOwner {}

pub struct UpdateLendingMarketOwnerInstructionAccounts {
    pub lending_market_owner_cached: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateLendingMarketOwner {
    type ArrangedAccounts = UpdateLendingMarketOwnerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lending_market_owner_cached, lending_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateLendingMarketOwnerInstructionAccounts {
            lending_market_owner_cached: lending_market_owner_cached.pubkey,
            lending_market: lending_market.pubkey,
        })
    }
}
