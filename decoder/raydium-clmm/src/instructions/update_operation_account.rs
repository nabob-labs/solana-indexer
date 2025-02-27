use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x7f467728bce33d07")]
pub struct UpdateOperationAccount {
    pub param: u8,
    pub keys: Vec<solana_sdk::pubkey::Pubkey>,
}

pub struct UpdateOperationAccountInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub operation_state: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateOperationAccount {
    type ArrangedAccounts = UpdateOperationAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, operation_state, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateOperationAccountInstructionAccounts {
            owner: owner.pubkey,
            operation_state: operation_state.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
