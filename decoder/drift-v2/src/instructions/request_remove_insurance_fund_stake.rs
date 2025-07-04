use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x8e46cc5c496ab434")]
pub struct RequestRemoveInsuranceFundStake {
    pub market_index: u16,
    pub amount: u64,
}

pub struct RequestRemoveInsuranceFundStakeInstructionAccounts {
    pub spot_market: solana_pubkey::Pubkey,
    pub insurance_fund_stake: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub insurance_fund_vault: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for RequestRemoveInsuranceFundStake {
    type ArrangedAccounts = RequestRemoveInsuranceFundStakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [spot_market, insurance_fund_stake, user_stats, authority, insurance_fund_vault, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RequestRemoveInsuranceFundStakeInstructionAccounts {
            spot_market: spot_market.pubkey,
            insurance_fund_stake: insurance_fund_stake.pubkey,
            user_stats: user_stats.pubkey,
            authority: authority.pubkey,
            insurance_fund_vault: insurance_fund_vault.pubkey,
        })
    }
}
