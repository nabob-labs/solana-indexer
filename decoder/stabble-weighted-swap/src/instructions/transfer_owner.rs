use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xf519ddaf6ae5e12d")]
pub struct TransferOwner {
    pub new_owner: solana_sdk::pubkey::Pubkey,
}

pub struct TransferOwnerInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for TransferOwner {
    type ArrangedAccounts = TransferOwnerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, pool, _remaining @ ..] = accounts else {
            return None;
        };

        Some(TransferOwnerInstructionAccounts {
            owner: owner.pubkey,
            pool: pool.pubkey,
        })
    }
}
