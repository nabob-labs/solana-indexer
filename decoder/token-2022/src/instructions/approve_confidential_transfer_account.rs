use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x1b")]
pub struct ApproveConfidentialTransferAccount {
    pub confidential_transfer_discriminator: u8,
}

pub struct ApproveConfidentialTransferAccountInstructionAccounts {
    pub token: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ApproveConfidentialTransferAccount {
    type ArrangedAccounts = ApproveConfidentialTransferAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token, mint, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ApproveConfidentialTransferAccountInstructionAccounts {
            token: token.pubkey,
            mint: mint.pubkey,
            authority: authority.pubkey,
        })
    }
}
