use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x49abb84b1e38c6df")]
pub struct SetStakeDelegated {
    pub new_amount: u64,
}

pub struct SetStakeDelegatedInstructionAccounts {
    pub delegate_authority: solana_pubkey::Pubkey,
    pub user_state: solana_pubkey::Pubkey,
    pub farm_state: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetStakeDelegated {
    type ArrangedAccounts = SetStakeDelegatedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [delegate_authority, user_state, farm_state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetStakeDelegatedInstructionAccounts {
            delegate_authority: delegate_authority.pubkey,
            user_state: user_state.pubkey,
            farm_state: farm_state.pubkey,
        })
    }
}
