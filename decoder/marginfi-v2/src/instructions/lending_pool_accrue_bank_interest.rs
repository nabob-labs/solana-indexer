use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x6cc91e572f4161bc")]
pub struct LendingPoolAccrueBankInterest {}

pub struct LendingPoolAccrueBankInterestInstructionAccounts {
    pub marginfi_group: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for LendingPoolAccrueBankInterest {
    type ArrangedAccounts = LendingPoolAccrueBankInterestInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_group, bank, _remaining @ ..] = accounts else {
            return None;
        };

        Some(LendingPoolAccrueBankInterestInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            bank: bank.pubkey,
        })
    }
}
