use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x19")]
pub struct InitializeMintCloseAuthority {
    pub close_authority: Option<solana_pubkey::Pubkey>,
}

pub struct InitializeMintCloseAuthorityInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeMintCloseAuthority {
    type ArrangedAccounts = InitializeMintCloseAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeMintCloseAuthorityInstructionAccounts { mint: mint.pubkey })
    }
}
