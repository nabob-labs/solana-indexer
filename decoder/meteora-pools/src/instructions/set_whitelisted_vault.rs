use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0c945e2a373953f7")]
pub struct SetWhitelistedVault {
    pub whitelisted_vault: solana_pubkey::Pubkey,
}

pub struct SetWhitelistedVaultInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetWhitelistedVault {
    type ArrangedAccounts = SetWhitelistedVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetWhitelistedVaultInstructionAccounts {
            pool: pool.pubkey,
            admin: admin.pubkey,
        })
    }
}
