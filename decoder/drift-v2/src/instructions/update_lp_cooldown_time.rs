use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xc6855829f1773d0e")]
pub struct UpdateLpCooldownTime {
    pub lp_cooldown_time: u64,
}

pub struct UpdateLpCooldownTimeInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateLpCooldownTime {
    type ArrangedAccounts = UpdateLpCooldownTimeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateLpCooldownTimeInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
