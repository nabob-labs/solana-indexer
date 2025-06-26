use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x01000000")]
pub struct Assign {
    pub program_address: solana_pubkey::Pubkey,
}

pub struct AssignInstructionAccounts {
    pub account: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Assign {
    type ArrangedAccounts = AssignInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AssignInstructionAccounts {
            account: account.pubkey,
        })
    }
}
