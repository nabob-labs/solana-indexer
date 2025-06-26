use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x078da23c47731a92")]
pub struct ApproveStrategy {}

pub struct ApproveStrategyInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub vault: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub strategy: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ApproveStrategy {
    type ArrangedAccounts = ApproveStrategyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, vault, admin, strategy, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ApproveStrategyInstructionAccounts {
            pool: pool.pubkey,
            vault: vault.pubkey,
            admin: admin.pubkey,
            strategy: strategy.pubkey,
        })
    }
}
