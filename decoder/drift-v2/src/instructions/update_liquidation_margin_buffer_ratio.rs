use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x84e0f3a09a5261d7")]
pub struct UpdateLiquidationMarginBufferRatio {
    pub liquidation_margin_buffer_ratio: u32,
}

pub struct UpdateLiquidationMarginBufferRatioInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateLiquidationMarginBufferRatio {
    type ArrangedAccounts = UpdateLiquidationMarginBufferRatioInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateLiquidationMarginBufferRatioInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
