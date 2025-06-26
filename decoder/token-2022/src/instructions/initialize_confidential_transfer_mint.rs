use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x1b")]
pub struct InitializeConfidentialTransferMint {
    pub confidential_transfer_discriminator: u8,
    pub authority: Option<solana_pubkey::Pubkey>,
    pub auto_approve_new_accounts: bool,
    pub auditor_elgamal_pubkey: Option<solana_pubkey::Pubkey>,
}

pub struct InitializeConfidentialTransferMintInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeConfidentialTransferMint {
    type ArrangedAccounts = InitializeConfidentialTransferMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeConfidentialTransferMintInstructionAccounts { mint: mint.pubkey })
    }
}
