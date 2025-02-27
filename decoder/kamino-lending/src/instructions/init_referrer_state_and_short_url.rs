use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xa513197f64371f5a")]
pub struct InitReferrerStateAndShortUrl {
    pub short_url: String,
}

pub struct InitReferrerStateAndShortUrlInstructionAccounts {
    pub referrer: solana_sdk::pubkey::Pubkey,
    pub referrer_state: solana_sdk::pubkey::Pubkey,
    pub referrer_short_url: solana_sdk::pubkey::Pubkey,
    pub referrer_user_metadata: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitReferrerStateAndShortUrl {
    type ArrangedAccounts = InitReferrerStateAndShortUrlInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [referrer, referrer_state, referrer_short_url, referrer_user_metadata, rent, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitReferrerStateAndShortUrlInstructionAccounts {
            referrer: referrer.pubkey,
            referrer_state: referrer_state.pubkey,
            referrer_short_url: referrer_short_url.pubkey,
            referrer_user_metadata: referrer_user_metadata.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
