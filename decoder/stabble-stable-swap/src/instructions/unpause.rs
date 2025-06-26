use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xa99004260a8dbcff")]
pub struct Unpause {}

pub struct UnpauseInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Unpause {
    type ArrangedAccounts = UnpauseInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, pool, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UnpauseInstructionAccounts {
            owner: owner.pubkey,
            pool: pool.pubkey,
        })
    }
}
