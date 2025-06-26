use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x82576c062ee0757b")]
pub struct UpdatePoolStatus {
    pub status: u8,
}

#[derive(Debug, PartialEq)]
pub struct UpdatePoolStatusInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdatePoolStatus {
    type ArrangedAccounts = UpdatePoolStatusInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, pool_state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdatePoolStatusInstructionAccounts {
            authority: authority.pubkey,
            pool_state: pool_state.pubkey,
        })
    }
}
