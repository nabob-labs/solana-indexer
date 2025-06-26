use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x16")]
pub struct InitializeImmutableOwner {}

pub struct InitializeImmutableOwnerInstructionAccounts {
    pub account: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeImmutableOwner {
    type ArrangedAccounts = InitializeImmutableOwnerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeImmutableOwnerInstructionAccounts {
            account: account.pubkey,
        })
    }
}
