use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x39")]
pub struct CloseAccounts {}

pub struct CloseAccountsInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub edition: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub destination: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CloseAccounts {
    type ArrangedAccounts = CloseAccountsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, edition, mint, authority, destination, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CloseAccountsInstructionAccounts {
            metadata: metadata.pubkey,
            edition: edition.pubkey,
            mint: mint.pubkey,
            authority: authority.pubkey,
            destination: destination.pubkey,
        })
    }
}
