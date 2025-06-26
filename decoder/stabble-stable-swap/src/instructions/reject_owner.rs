use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xeecec6d733b285e4")]
pub struct RejectOwner {}

pub struct RejectOwnerInstructionAccounts {
    pub pending_owner: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for RejectOwner {
    type ArrangedAccounts = RejectOwnerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pending_owner, pool, _remaining @ ..] = accounts else {
            return None;
        };

        Some(RejectOwnerInstructionAccounts {
            pending_owner: pending_owner.pubkey,
            pool: pool.pubkey,
        })
    }
}
