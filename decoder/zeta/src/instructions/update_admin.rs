use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xa1b028d53cb8b3e4")]
pub struct UpdateAdmin {}

pub struct UpdateAdminInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub new_admin: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateAdmin {
    type ArrangedAccounts = UpdateAdminInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, new_admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateAdminInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
            new_admin: new_admin.pubkey,
        })
    }
}
