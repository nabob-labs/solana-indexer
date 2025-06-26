use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x697cc96a9902089c")]
pub struct LendingAccountEndFlashloan {}

pub struct LendingAccountEndFlashloanInstructionAccounts {
    pub marginfi_account: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for LendingAccountEndFlashloan {
    type ArrangedAccounts = LendingAccountEndFlashloanInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_account, signer, _remaining @ ..] = accounts else {
            return None;
        };

        Some(LendingAccountEndFlashloanInstructionAccounts {
            marginfi_account: marginfi_account.pubkey,
            signer: signer.pubkey,
        })
    }
}
