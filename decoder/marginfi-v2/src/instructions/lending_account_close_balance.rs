use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xf5362904f3ca1f11")]
pub struct LendingAccountCloseBalance {}

pub struct LendingAccountCloseBalanceInstructionAccounts {
    pub marginfi_group: solana_pubkey::Pubkey,
    pub marginfi_account: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for LendingAccountCloseBalance {
    type ArrangedAccounts = LendingAccountCloseBalanceInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_group, marginfi_account, signer, bank, _remaining @ ..] = accounts else {
            return None;
        };

        Some(LendingAccountCloseBalanceInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            marginfi_account: marginfi_account.pubkey,
            signer: signer.pubkey,
            bank: bank.pubkey,
        })
    }
}
