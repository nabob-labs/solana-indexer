use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x04")]
pub struct Approve {
    pub amount: u64,
}

pub struct ApproveAccounts {
    pub source: solana_pubkey::Pubkey,
    pub delegate: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Approve {
    type ArrangedAccounts = ApproveAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [source, delegate, owner, remaining_accounts @ ..] = accounts else {
            return None;
        };

        Some(ApproveAccounts {
            source: source.pubkey,
            delegate: delegate.pubkey,
            owner: owner.pubkey,
            remaining_accounts: remaining_accounts.to_vec(),
        })
    }
}
