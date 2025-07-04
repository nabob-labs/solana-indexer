use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x98a06b94f5be7fe0")]
pub struct CreateStrategy {
    pub amp_min_factor: u16,
    pub amp_max_factor: u16,
    pub ramp_min_step: u16,
    pub ramp_max_step: u16,
    pub ramp_min_duration: u32,
    pub ramp_max_duration: u32,
}

pub struct CreateStrategyInstructionAccounts {
    pub owner_only: solana_pubkey::Pubkey,
    pub strategy: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CreateStrategy {
    type ArrangedAccounts = CreateStrategyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner_only, strategy, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CreateStrategyInstructionAccounts {
            owner_only: owner_only.pubkey,
            strategy: strategy.pubkey,
        })
    }
}
