use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x17")]
pub struct AmountToUiAmount {
    pub amount: u64,
}

pub struct AmountToUiAmountInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for AmountToUiAmount {
    type ArrangedAccounts = AmountToUiAmountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AmountToUiAmountInstructionAccounts { mint: mint.pubkey })
    }
}
