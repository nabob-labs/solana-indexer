use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x41b1d749352d632f")]
pub struct TransferOwnership {
    pub new_owner: solana_pubkey::Pubkey,
}

pub struct TransferOwnershipInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub user_state: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for TransferOwnership {
    type ArrangedAccounts = TransferOwnershipInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, user_state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(TransferOwnershipInstructionAccounts {
            owner: owner.pubkey,
            user_state: user_state.pubkey,
        })
    }
}
