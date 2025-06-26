use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x07160c53f22b3079")]
pub struct TransferRewardOwner {
    pub new_owner: solana_pubkey::Pubkey,
}

pub struct TransferRewardOwnerInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for TransferRewardOwner {
    type ArrangedAccounts = TransferRewardOwnerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, pool_state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(TransferRewardOwnerInstructionAccounts {
            authority: authority.pubkey,
            pool_state: pool_state.pubkey,
        })
    }
}
