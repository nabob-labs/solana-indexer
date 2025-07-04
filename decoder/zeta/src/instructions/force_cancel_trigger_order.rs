use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x78ecd81cc04fffbc")]
pub struct ForceCancelTriggerOrder {
    pub trigger_order_bit: u8,
    pub enforce_tpsl_conditions: bool,
}

pub struct ForceCancelTriggerOrderInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub trigger_order: solana_pubkey::Pubkey,
    pub margin_account: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ForceCancelTriggerOrder {
    type ArrangedAccounts = ForceCancelTriggerOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, trigger_order, margin_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ForceCancelTriggerOrderInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
            trigger_order: trigger_order.pubkey,
            margin_account: margin_account.pubkey,
        })
    }
}
