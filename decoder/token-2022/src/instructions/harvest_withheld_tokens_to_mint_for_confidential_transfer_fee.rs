use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x25")]
pub struct HarvestWithheldTokensToMintForConfidentialTransferFee {
    pub confidential_transfer_fee_discriminator: u8,
}

pub struct HarvestWithheldTokensToMintForConfidentialTransferFeeInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts
    for HarvestWithheldTokensToMintForConfidentialTransferFee
{
    type ArrangedAccounts =
        HarvestWithheldTokensToMintForConfidentialTransferFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(
            HarvestWithheldTokensToMintForConfidentialTransferFeeInstructionAccounts {
                mint: mint.pubkey,
            },
        )
    }
}
