use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x24")]
pub struct InitializeTransferHook {
    pub transfer_hook_discriminator: u8,
    pub authority: Option<solana_pubkey::Pubkey>,
    pub program_id: Option<solana_pubkey::Pubkey>,
}

pub struct InitializeTransferHookInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeTransferHook {
    type ArrangedAccounts = InitializeTransferHookInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeTransferHookInstructionAccounts { mint: mint.pubkey })
    }
}
