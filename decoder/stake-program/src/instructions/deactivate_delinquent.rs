use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0671c68ae4a39fdd")]
pub struct DeactivateDelinquent {}

pub struct DeactivateDelinquentInstructionAccounts {
    pub stake: solana_sdk::pubkey::Pubkey,
    pub vote: solana_sdk::pubkey::Pubkey,
    pub reference_vote: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for DeactivateDelinquent {
    type ArrangedAccounts = DeactivateDelinquentInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [stake, vote, reference_vote, _remaining @ ..] = accounts else {
            return None;
        };

        Some(DeactivateDelinquentInstructionAccounts {
            stake: stake.pubkey,
            vote: vote.pubkey,
            reference_vote: reference_vote.pubkey,
        })
    }
}
