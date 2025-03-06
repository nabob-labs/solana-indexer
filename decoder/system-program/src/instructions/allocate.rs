use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x08")]
pub struct Allocate {
    pub space: u64,
}

pub struct AllocateAccounts {
    pub new_account: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Allocate {
    type ArrangedAccounts = AllocateAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let new_account = accounts.get(0)?;

        Some(AllocateAccounts {
            new_account: new_account.pubkey,
        })
    }
}
