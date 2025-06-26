use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0e8321dc51bab46b")]
pub struct LendingAccountStartFlashloan {
    pub end_index: u64,
}

pub struct LendingAccountStartFlashloanInstructionAccounts {
    pub marginfi_account: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
    pub ixs_sysvar: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for LendingAccountStartFlashloan {
    type ArrangedAccounts = LendingAccountStartFlashloanInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_account, signer, ixs_sysvar, _remaining @ ..] = accounts else {
            return None;
        };

        Some(LendingAccountStartFlashloanInstructionAccounts {
            marginfi_account: marginfi_account.pubkey,
            signer: signer.pubkey,
            ixs_sysvar: ixs_sysvar.pubkey,
        })
    }
}
