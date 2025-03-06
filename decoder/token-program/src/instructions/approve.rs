use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x04")]
pub struct Approve {
    pub amount: u64,
}

pub struct ApproveAccounts {
    pub source: solana_sdk::pubkey::Pubkey,
    pub delegate: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Approve {
    type ArrangedAccounts = ApproveAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let source = accounts.get(0)?;
        let delegate = accounts.get(1)?;
        let owner = accounts.get(2)?;

        Some(ApproveAccounts {
            source: source.pubkey,
            delegate: delegate.pubkey,
            owner: owner.pubkey,
            remaining_accounts: accounts.get(3..).unwrap_or_default().to_vec(),
        })
    }
}
