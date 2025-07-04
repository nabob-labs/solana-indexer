use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xd6838ab790c2ac2a")]
pub struct RefreshFarm {}

pub struct RefreshFarmInstructionAccounts {
    pub farm_state: solana_pubkey::Pubkey,
    pub scope_prices: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for RefreshFarm {
    type ArrangedAccounts = RefreshFarmInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [farm_state, scope_prices, _remaining @ ..] = accounts else {
            return None;
        };

        Some(RefreshFarmInstructionAccounts {
            farm_state: farm_state.pubkey,
            scope_prices: scope_prices.pubkey,
        })
    }
}
