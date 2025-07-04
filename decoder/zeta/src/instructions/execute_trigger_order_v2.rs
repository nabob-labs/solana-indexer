use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x05e4307708d6b796")]
pub struct ExecuteTriggerOrderV2 {
    pub trigger_order_bit: u8,
}

pub struct ExecuteTriggerOrderV2InstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub trigger_order: solana_pubkey::Pubkey,
    pub place_order_accounts: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ExecuteTriggerOrderV2 {
    type ArrangedAccounts = ExecuteTriggerOrderV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, trigger_order, place_order_accounts, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ExecuteTriggerOrderV2InstructionAccounts {
            payer: payer.pubkey,
            trigger_order: trigger_order.pubkey,
            place_order_accounts: place_order_accounts.pubkey,
        })
    }
}
