use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x25")]
pub struct InitializeConfidentialTransferFee {
    pub confidential_transfer_fee_discriminator: u8,
    pub authority: Option<solana_pubkey::Pubkey>,
    pub withdraw_withheld_authority_el_gamal_pubkey: Option<solana_pubkey::Pubkey>,
}

pub struct InitializeConfidentialTransferFeeInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeConfidentialTransferFee {
    type ArrangedAccounts = InitializeConfidentialTransferFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeConfidentialTransferFeeInstructionAccounts { mint: mint.pubkey })
    }
}
