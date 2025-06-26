use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xa13a88aef2df9cb0")]
pub struct LendingAccountSettleEmissions {}

pub struct LendingAccountSettleEmissionsInstructionAccounts {
    pub marginfi_account: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for LendingAccountSettleEmissions {
    type ArrangedAccounts = LendingAccountSettleEmissionsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_account, bank, _remaining @ ..] = accounts else {
            return None;
        };

        Some(LendingAccountSettleEmissionsInstructionAccounts {
            marginfi_account: marginfi_account.pubkey,
            bank: bank.pubkey,
        })
    }
}
