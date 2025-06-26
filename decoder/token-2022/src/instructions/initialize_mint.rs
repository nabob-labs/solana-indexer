use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x00")]
pub struct InitializeMint {
    pub decimals: u8,
    pub mint_authority: solana_pubkey::Pubkey,
    pub freeze_authority: Option<solana_pubkey::Pubkey>,
}

pub struct InitializeMintInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeMint {
    type ArrangedAccounts = InitializeMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, rent, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeMintInstructionAccounts {
            mint: mint.pubkey,
            rent: rent.pubkey,
        })
    }
}
