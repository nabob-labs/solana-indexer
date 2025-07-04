use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xa1695801edddd8cb")]
pub struct UpdateTokenGroupUpdateAuthority {
    pub new_update_authority: Option<solana_pubkey::Pubkey>,
}

pub struct UpdateTokenGroupUpdateAuthorityInstructionAccounts {
    pub group: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateTokenGroupUpdateAuthority {
    type ArrangedAccounts = UpdateTokenGroupUpdateAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [group, update_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateTokenGroupUpdateAuthorityInstructionAccounts {
            group: group.pubkey,
            update_authority: update_authority.pubkey,
        })
    }
}
