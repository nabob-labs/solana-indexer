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
    pub source: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub delegate: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ApproveChecked {
    type ArrangedAccounts = ApproveCheckedAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [source, mint, delegate, owner, remaining_accounts @ ..] = accounts else {
            return None;
        };

        Some(ApproveCheckedAccounts {
            source: source.pubkey,
            mint: mint.pubkey,
            delegate: delegate.pubkey,
            owner: owner.pubkey,
            remaining_accounts: remaining_accounts.to_vec(),
        })
    }
}
