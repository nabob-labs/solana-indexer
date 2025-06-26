use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x38ee12cfc1528aae")]
pub struct SetAccountFlag {
    pub flag: u64,
}

pub struct SetAccountFlagInstructionAccounts {
    pub marginfi_group: solana_pubkey::Pubkey,
    pub marginfi_account: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetAccountFlag {
    type ArrangedAccounts = SetAccountFlagInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_group, marginfi_account, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetAccountFlagInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            marginfi_account: marginfi_account.pubkey,
            admin: admin.pubkey,
        })
    }
}
