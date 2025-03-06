use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x15")]
pub struct GetAccountDataSize {}

pub struct GetAccountDataSizeAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for GetAccountDataSize {
    type ArrangedAccounts = GetAccountDataSizeAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.get(0)?;

        Some(GetAccountDataSizeAccounts { mint: mint.pubkey })
    }
}
