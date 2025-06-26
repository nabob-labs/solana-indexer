use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x22")]
pub struct EnableCpiGuard {
    pub cpi_guard_discriminator: u8,
}

pub struct EnableCpiGuardInstructionAccounts {
    pub token: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for EnableCpiGuard {
    type ArrangedAccounts = EnableCpiGuardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token, owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(EnableCpiGuardInstructionAccounts {
            token: token.pubkey,
            owner: owner.pubkey,
        })
    }
}
