use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x05")]
pub struct Revoke {}

pub struct RevokeAccounts {
    pub source: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Revoke {
    type ArrangedAccounts = RevokeAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let source = accounts.get(0)?;
        let owner = accounts.get(1)?;

        Some(RevokeAccounts {
            source: source.pubkey,
            owner: owner.pubkey,
            remaining_accounts: accounts.get(2..).unwrap_or_default().to_vec(),
        })
    }
}
