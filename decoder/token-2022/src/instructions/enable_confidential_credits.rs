use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x1b")]
pub struct EnableConfidentialCredits {
    pub confidential_transfer_discriminator: u8,
}

pub struct EnableConfidentialCreditsInstructionAccounts {
    pub token: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for EnableConfidentialCredits {
    type ArrangedAccounts = EnableConfidentialCreditsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(EnableConfidentialCreditsInstructionAccounts {
            token: token.pubkey,
            authority: authority.pubkey,
        })
    }
}
