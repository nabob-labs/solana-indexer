use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xdb08f660a9795b6e")]
pub struct UpdateSerumVault {}

pub struct UpdateSerumVaultInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub srm_vault: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateSerumVault {
    type ArrangedAccounts = UpdateSerumVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, srm_vault, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateSerumVaultInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
            srm_vault: srm_vault.pubkey,
        })
    }
}
