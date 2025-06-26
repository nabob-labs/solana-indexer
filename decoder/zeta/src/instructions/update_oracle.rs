use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x7029d112f8e2fcbc")]
pub struct UpdateOracle {}

pub struct UpdateOracleInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub zeta_group: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateOracle {
    type ArrangedAccounts = UpdateOracleInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, zeta_group, admin, oracle, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateOracleInstructionAccounts {
            state: state.pubkey,
            zeta_group: zeta_group.pubkey,
            admin: admin.pubkey,
            oracle: oracle.pubkey,
        })
    }
}
