use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xf34fcce4e3d064f4")]
pub struct PostMultiPythPullOracleUpdatesAtomic {
    pub params: Vec<u8>,
}

pub struct PostMultiPythPullOracleUpdatesAtomicInstructionAccounts {
    pub keeper: solana_pubkey::Pubkey,
    pub pyth_solana_receiver: solana_pubkey::Pubkey,
    pub guardian_set: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for PostMultiPythPullOracleUpdatesAtomic {
    type ArrangedAccounts = PostMultiPythPullOracleUpdatesAtomicInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, pyth_solana_receiver, guardian_set, _remaining @ ..] = accounts else {
            return None;
        };

        Some(PostMultiPythPullOracleUpdatesAtomicInstructionAccounts {
            keeper: keeper.pubkey,
            pyth_solana_receiver: pyth_solana_receiver.pubkey,
            guardian_set: guardian_set.pubkey,
        })
    }
}
