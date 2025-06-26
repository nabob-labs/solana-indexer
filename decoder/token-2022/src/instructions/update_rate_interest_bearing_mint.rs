use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x21")]
pub struct UpdateRateInterestBearingMint {
    pub interest_bearing_mint_discriminator: u8,
    pub rate: i16,
}

pub struct UpdateRateInterestBearingMintInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
    pub rate_authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateRateInterestBearingMint {
    type ArrangedAccounts = UpdateRateInterestBearingMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, rate_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateRateInterestBearingMintInstructionAccounts {
            mint: mint.pubkey,
            rate_authority: rate_authority.pubkey,
        })
    }
}
