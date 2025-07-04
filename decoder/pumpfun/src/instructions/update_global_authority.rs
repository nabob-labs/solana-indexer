use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe3b54ac4d01561d5")]
pub struct UpdateGlobalAuthority {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateGlobalAuthorityInstructionAccounts {
    pub global: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub new_authority: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateGlobalAuthority {
    type ArrangedAccounts = UpdateGlobalAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [global, authority, new_authority, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(UpdateGlobalAuthorityInstructionAccounts {
            global: global.pubkey,
            authority: authority.pubkey,
            new_authority: new_authority.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
