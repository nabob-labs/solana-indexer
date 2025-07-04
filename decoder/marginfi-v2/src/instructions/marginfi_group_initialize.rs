use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xff43431a5e1f2214")]
pub struct MarginfiGroupInitialize {}

pub struct MarginfiGroupInitializeInstructionAccounts {
    pub marginfi_group: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for MarginfiGroupInitialize {
    type ArrangedAccounts = MarginfiGroupInitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_group, admin, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(MarginfiGroupInitializeInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            admin: admin.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
