

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0x3d9464468f6b110d")]
pub struct UpdateReserveConfig{
    pub mode: u64,
    pub value: Vec<u8>,
    pub skip_validation: bool,
}

pub struct UpdateReserveConfigInstructionAccounts {
    pub lending_market_owner: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateReserveConfig {
    type ArrangedAccounts = UpdateReserveConfigInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let lending_market_owner = accounts.get(0)?;
        let lending_market = accounts.get(1)?;
        let reserve = accounts.get(2)?;

        Some(UpdateReserveConfigInstructionAccounts {
            lending_market_owner: lending_market_owner.pubkey,
            lending_market: lending_market.pubkey,
            reserve: reserve.pubkey,
        })
    }
}