use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x29")]
pub struct UpdateGroupMemberPointer {
    pub group_member_pointer_discriminator: u8,
    pub member_address: Option<solana_pubkey::Pubkey>,
}

pub struct UpdateGroupMemberPointerInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
    pub group_member_pointer_authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateGroupMemberPointer {
    type ArrangedAccounts = UpdateGroupMemberPointerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, group_member_pointer_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateGroupMemberPointerInstructionAccounts {
            mint: mint.pubkey,
            group_member_pointer_authority: group_member_pointer_authority.pubkey,
        })
    }
}
