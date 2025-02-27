use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x1a")]
pub struct InitializeTransferFeeConfig {
    pub transfer_fee_discriminator: u8,
    pub transfer_fee_config_authority: Option<solana_sdk::pubkey::Pubkey>,
    pub withdraw_withheld_authority: Option<solana_sdk::pubkey::Pubkey>,
    pub transfer_fee_basis_points: u16,
    pub maximum_fee: u64,
}

pub struct InitializeTransferFeeConfigInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeTransferFeeConfig {
    type ArrangedAccounts = InitializeTransferFeeConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeTransferFeeConfigInstructionAccounts { mint: mint.pubkey })
    }
}
