use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x53a0fcfa817431df")]
pub struct UpdateExchangeStatus {
    pub exchange_status: u8,
}

pub struct UpdateExchangeStatusInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateExchangeStatus {
    type ArrangedAccounts = UpdateExchangeStatusInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateExchangeStatusInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
