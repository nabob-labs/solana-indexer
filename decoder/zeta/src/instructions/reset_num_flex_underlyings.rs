use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x3013fed1c8d3313d")]
pub struct ResetNumFlexUnderlyings {}

pub struct ResetNumFlexUnderlyingsInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ResetNumFlexUnderlyings {
    type ArrangedAccounts = ResetNumFlexUnderlyingsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ResetNumFlexUnderlyingsInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
        })
    }
}
