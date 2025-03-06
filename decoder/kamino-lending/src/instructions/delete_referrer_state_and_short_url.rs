

use solana_indexer_core::{IndexerDeserialize, borsh};


#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[indexer(discriminator = "0x99b9631ce4b3bb96")]
pub struct DeleteReferrerStateAndShortUrl{
}

pub struct DeleteReferrerStateAndShortUrlInstructionAccounts {
    pub referrer: solana_sdk::pubkey::Pubkey,
    pub referrer_state: solana_sdk::pubkey::Pubkey,
    pub short_url: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for DeleteReferrerStateAndShortUrl {
    type ArrangedAccounts = DeleteReferrerStateAndShortUrlInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let referrer = accounts.get(0)?;
        let referrer_state = accounts.get(1)?;
        let short_url = accounts.get(2)?;
        let rent = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(DeleteReferrerStateAndShortUrlInstructionAccounts {
            referrer: referrer.pubkey,
            referrer_state: referrer_state.pubkey,
            short_url: short_url.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}