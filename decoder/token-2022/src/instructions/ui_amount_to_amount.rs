use alloc::string::String;
use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x18")]
pub struct UiAmountToAmount {
    pub ui_amount: String,
}

pub struct UiAmountToAmountInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UiAmountToAmount {
    type ArrangedAccounts = UiAmountToAmountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UiAmountToAmountInstructionAccounts { mint: mint.pubkey })
    }
}
