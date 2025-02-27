use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x24")]
pub struct UpdateTransferHook {
    pub transfer_hook_discriminator: u8,
    pub program_id: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct UpdateTransferHookInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateTransferHook {
    type ArrangedAccounts = UpdateTransferHookInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateTransferHookInstructionAccounts {
            mint: mint.pubkey,
            authority: authority.pubkey,
        })
    }
}
