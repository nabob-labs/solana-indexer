use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xb79b2d00e255d545")]
pub struct DisableUserHighLeverageMode {}

pub struct DisableUserHighLeverageModeInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub high_leverage_mode_config: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for DisableUserHighLeverageMode {
    type ArrangedAccounts = DisableUserHighLeverageModeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, authority, user, high_leverage_mode_config, _remaining @ ..] = accounts else {
            return None;
        };

        Some(DisableUserHighLeverageModeInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
            user: user.pubkey,
            high_leverage_mode_config: high_leverage_mode_config.pubkey,
        })
    }
}
