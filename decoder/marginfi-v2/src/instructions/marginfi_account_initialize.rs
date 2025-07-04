use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x2b4e3dff9434f99a")]
pub struct MarginfiAccountInitialize {}

pub struct MarginfiAccountInitializeInstructionAccounts {
    pub marginfi_group: solana_pubkey::Pubkey,
    pub marginfi_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub fee_payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for MarginfiAccountInitialize {
    type ArrangedAccounts = MarginfiAccountInitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_group, marginfi_account, authority, fee_payer, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(MarginfiAccountInitializeInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            marginfi_account: marginfi_account.pubkey,
            authority: authority.pubkey,
            fee_payer: fee_payer.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
