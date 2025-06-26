use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x8006e48337a134a9")]
pub struct EnableOrDisablePool {
    pub enable: bool,
}

pub struct EnableOrDisablePoolInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for EnableOrDisablePool {
    type ArrangedAccounts = EnableOrDisablePoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(EnableOrDisablePoolInstructionAccounts {
            pool: pool.pubkey,
            admin: admin.pubkey,
        })
    }
}
