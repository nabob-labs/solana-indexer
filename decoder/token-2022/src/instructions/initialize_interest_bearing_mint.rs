use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x21")]
pub struct InitializeInterestBearingMint {
    pub interest_bearing_mint_discriminator: u8,
    pub rate_authority: Option<solana_pubkey::Pubkey>,
    pub rate: i16,
}

pub struct InitializeInterestBearingMintInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeInterestBearingMint {
    type ArrangedAccounts = InitializeInterestBearingMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeInterestBearingMintInstructionAccounts { mint: mint.pubkey })
    }
}
