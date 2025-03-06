use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x76d7d69db6e5d0e4")]
pub struct SetDefaultFeeRate {
    pub default_fee_rate: u16,
}

pub struct SetDefaultFeeRateInstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub fee_tier: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetDefaultFeeRate {
    type ArrangedAccounts = SetDefaultFeeRateInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let whirlpools_config = accounts.get(0)?;
        let fee_tier = accounts.get(1)?;
        let fee_authority = accounts.get(2)?;

        Some(SetDefaultFeeRateInstructionAccounts {
            whirlpools_config: whirlpools_config.pubkey,
            fee_tier: fee_tier.pubkey,
            fee_authority: fee_authority.pubkey,
        })
    }
}
