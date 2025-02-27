use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xeceeb045ef0ab5c1")]
pub struct RevertFill {}

pub struct RevertFillInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub filler: solana_sdk::pubkey::Pubkey,
    pub filler_stats: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for RevertFill {
    type ArrangedAccounts = RevertFillInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, authority, filler, filler_stats, _remaining @ ..] = accounts else {
            return None;
        };

        Some(RevertFillInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
            filler: filler.pubkey,
            filler_stats: filler_stats.pubkey,
        })
    }
}
