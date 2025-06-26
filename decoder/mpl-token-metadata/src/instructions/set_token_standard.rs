use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x23")]
pub struct SetTokenStandard {}

pub struct SetTokenStandardInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub edition: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetTokenStandard {
    type ArrangedAccounts = SetTokenStandardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, update_authority, mint, edition, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetTokenStandardInstructionAccounts {
            metadata: metadata.pubkey,
            update_authority: update_authority.pubkey,
            mint: mint.pubkey,
            edition: edition.pubkey,
        })
    }
}
