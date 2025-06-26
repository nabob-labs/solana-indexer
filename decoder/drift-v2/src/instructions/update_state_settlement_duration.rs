use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x6144c7eb83503dad")]
pub struct UpdateStateSettlementDuration {
    pub settlement_duration: u16,
}

pub struct UpdateStateSettlementDurationInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateStateSettlementDuration {
    type ArrangedAccounts = UpdateStateSettlementDurationInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateStateSettlementDurationInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
