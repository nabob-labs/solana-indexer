use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x8cf669a550558f12")]
pub struct WithdrawHostTip {}

pub struct WithdrawHostTipInstructionAccounts {
    pub admin_authority: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub pda_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for WithdrawHostTip {
    type ArrangedAccounts = WithdrawHostTipInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin_authority, global_config, pda_authority, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawHostTipInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            global_config: global_config.pubkey,
            pda_authority: pda_authority.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
