use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe60921cae4d1b462")]
pub struct UpdateOracleBackupFeed {}

pub struct UpdateOracleBackupFeedInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub zeta_group: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateOracleBackupFeed {
    type ArrangedAccounts = UpdateOracleBackupFeedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, zeta_group, admin, oracle, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateOracleBackupFeedInstructionAccounts {
            state: state.pubkey,
            zeta_group: zeta_group.pubkey,
            admin: admin.pubkey,
            oracle: oracle.pubkey,
        })
    }
}
