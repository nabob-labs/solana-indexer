use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0d")]
pub struct ApproveChecked {
    pub amount: u64,
    pub decimals: u8,
}

pub struct ApproveCheckedInstructionAccounts {
    pub source: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub delegate: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ApproveChecked {
    type ArrangedAccounts = ApproveCheckedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [source, mint, delegate, owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ApproveCheckedInstructionAccounts {
            source: source.pubkey,
            mint: mint.pubkey,
            delegate: delegate.pubkey,
            owner: owner.pubkey,
        })
    }
}
