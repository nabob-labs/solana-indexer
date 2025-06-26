use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x1a")]
pub struct SetTransferFee {
    pub transfer_fee_discriminator: u8,
    pub transfer_fee_basis_points: u16,
    pub maximum_fee: u64,
}

pub struct SetTransferFeeInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
    pub transfer_fee_config_authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetTransferFee {
    type ArrangedAccounts = SetTransferFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, transfer_fee_config_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetTransferFeeInstructionAccounts {
            mint: mint.pubkey,
            transfer_fee_config_authority: transfer_fee_config_authority.pubkey,
        })
    }
}
