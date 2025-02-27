use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x03")]
pub struct CreateAccountWithSeed {
    pub base: solana_sdk::pubkey::Pubkey,
    pub seed: String,
    pub lamports: u64,
    pub space: u64,
    pub owner: solana_sdk::pubkey::Pubkey,
}

pub struct CreateAccountWithSeedAccounts {
    pub funding_account: solana_sdk::pubkey::Pubkey,
    pub created_account: solana_sdk::pubkey::Pubkey,
    pub base_account: Option<solana_sdk::pubkey::Pubkey>,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CreateAccountWithSeed {
    type ArrangedAccounts = CreateAccountWithSeedAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [funding_account, created_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CreateAccountWithSeedAccounts {
            funding_account: funding_account.pubkey,
            created_account: created_account.pubkey,
            base_account: accounts.get(2).cloned().map(|acc| acc.pubkey),
        })
    }
}
