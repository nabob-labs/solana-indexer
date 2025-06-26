use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x13")]
pub struct Collect {}

pub struct CollectInstructionAccounts {
    pub recipient1: solana_pubkey::Pubkey,
    pub recipient2: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Collect {
    type ArrangedAccounts = CollectInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [recipient1, recipient2, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CollectInstructionAccounts {
            recipient1: recipient1.pubkey,
            recipient2: recipient2.pubkey,
        })
    }
}
