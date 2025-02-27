use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
#[indexer(discriminator = "0x6dc64f7941caa18e")]
pub struct StubOracleSet {
    pub price: f64,
}

pub struct StubOracleSetInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for StubOracleSet {
    type ArrangedAccounts = StubOracleSetInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, oracle, _remaining @ ..] = accounts else {
            return None;
        };

        Some(StubOracleSetInstructionAccounts {
            owner: owner.pubkey,
            oracle: oracle.pubkey,
        })
    }
}
