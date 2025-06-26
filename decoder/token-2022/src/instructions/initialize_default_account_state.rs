use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x1c")]
pub struct InitializeDefaultAccountState {
    pub default_account_state_discriminator: u8,
    pub state: AccountState,
}

pub struct InitializeDefaultAccountStateInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeDefaultAccountState {
    type ArrangedAccounts = InitializeDefaultAccountStateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeDefaultAccountStateInstructionAccounts { mint: mint.pubkey })
    }
}
