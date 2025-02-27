use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x1a")]
pub struct HarvestWithheldTokensToMint {
    pub transfer_fee_discriminator: u8,
}

pub struct HarvestWithheldTokensToMintInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for HarvestWithheldTokensToMint {
    type ArrangedAccounts = HarvestWithheldTokensToMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(HarvestWithheldTokensToMintInstructionAccounts { mint: mint.pubkey })
    }
}
