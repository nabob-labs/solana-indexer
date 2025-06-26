use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x365393c67b61da48")]
pub struct TogglePaused {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TogglePausedInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for TogglePaused {
    type ArrangedAccounts = TogglePausedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, config, _remaining @ ..] = accounts else {
            return None;
        };

        Some(TogglePausedInstructionAccounts {
            authority: authority.pubkey,
            config: config.pubkey,
        })
    }
}
