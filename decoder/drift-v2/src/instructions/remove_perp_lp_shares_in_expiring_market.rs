use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x53fefd893b7a449c")]
pub struct RemovePerpLpSharesInExpiringMarket {
    pub shares_to_burn: u64,
    pub market_index: u16,
}

pub struct RemovePerpLpSharesInExpiringMarketInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for RemovePerpLpSharesInExpiringMarket {
    type ArrangedAccounts = RemovePerpLpSharesInExpiringMarketInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, user, _remaining @ ..] = accounts else {
            return None;
        };

        Some(RemovePerpLpSharesInExpiringMarketInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
        })
    }
}
