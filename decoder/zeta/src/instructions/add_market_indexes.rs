use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x5ef690af04a4e9fc")]
pub struct AddMarketIndexes {}

pub struct AddMarketIndexesInstructionAccounts {
    pub market_indexes: solana_sdk::pubkey::Pubkey,
    pub zeta_group: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for AddMarketIndexes {
    type ArrangedAccounts = AddMarketIndexesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [market_indexes, zeta_group, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AddMarketIndexesInstructionAccounts {
            market_indexes: market_indexes.pubkey,
            zeta_group: zeta_group.pubkey,
        })
    }
}
