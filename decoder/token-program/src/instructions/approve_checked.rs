use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0d")]
pub struct ApproveChecked {
    pub amount: u64,
    pub decimals: u8,
}

pub struct ApproveCheckedAccounts {
    pub source: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub delegate: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ApproveChecked {
    type ArrangedAccounts = ApproveCheckedAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let source = accounts.get(0)?;
        let mint = accounts.get(1)?;
        let delegate = accounts.get(2)?;
        let owner = accounts.get(3)?;

        Some(ApproveCheckedAccounts {
            source: source.pubkey,
            mint: mint.pubkey,
            delegate: delegate.pubkey,
            owner: owner.pubkey,
            remaining_accounts: accounts.get(4..).unwrap_or_default().to_vec(),
        })
    }
}
