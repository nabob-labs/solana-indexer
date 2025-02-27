use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x28")]
pub struct UpdateGroupPointer {
    pub group_pointer_discriminator: u8,
    pub group_address: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct UpdateGroupPointerInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub group_pointer_authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateGroupPointer {
    type ArrangedAccounts = UpdateGroupPointerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, group_pointer_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateGroupPointerInstructionAccounts {
            mint: mint.pubkey,
            group_pointer_authority: group_pointer_authority.pubkey,
        })
    }
}
