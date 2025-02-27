use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0f")]
pub struct Log {}

pub struct LogInstructionAccounts {
    pub log_authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Log {
    type ArrangedAccounts = LogInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [log_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(LogInstructionAccounts {
            log_authority: log_authority.pubkey,
        })
    }
}
