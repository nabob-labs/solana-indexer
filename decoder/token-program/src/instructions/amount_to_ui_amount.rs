use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x17")]
pub struct AmountToUiAmount {
    pub amount: u64,
}

pub struct AmountToUiAmountAccounts {
    pub mint: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for AmountToUiAmount {
    type ArrangedAccounts = AmountToUiAmountAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.first()?;

        Some(AmountToUiAmountAccounts { mint: mint.pubkey })
    }
}
