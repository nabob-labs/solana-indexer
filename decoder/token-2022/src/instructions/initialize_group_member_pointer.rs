use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x29")]
pub struct InitializeGroupMemberPointer {
    pub group_member_pointer_discriminator: u8,
    pub authority: Option<solana_sdk::pubkey::Pubkey>,
    pub member_address: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct InitializeGroupMemberPointerInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeGroupMemberPointer {
    type ArrangedAccounts = InitializeGroupMemberPointerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeGroupMemberPointerInstructionAccounts { mint: mint.pubkey })
    }
}
