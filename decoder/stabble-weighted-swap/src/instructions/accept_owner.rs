use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xb017291c176f0804")]
pub struct AcceptOwner {}

pub struct AcceptOwnerInstructionAccounts {
    pub pending_owner: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for AcceptOwner {
    type ArrangedAccounts = AcceptOwnerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pending_owner, pool, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AcceptOwnerInstructionAccounts {
            pending_owner: pending_owner.pubkey,
            pool: pool.pubkey,
        })
    }
}
