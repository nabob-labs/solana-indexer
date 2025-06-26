use solana_indexer_core::{borsh, deserialize::U64PrefixString, IndexerDeserialize};

#[derive(IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
#[indexer(discriminator = "0x0a000000")]
pub struct AssignWithSeed {
    pub base: solana_pubkey::Pubkey,
    pub seed: U64PrefixString,
    pub program_address: solana_pubkey::Pubkey,
}

pub struct AssignWithSeedInstructionAccounts {
    pub account: solana_pubkey::Pubkey,
    pub base_account: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for AssignWithSeed {
    type ArrangedAccounts = AssignWithSeedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, base_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(AssignWithSeedInstructionAccounts {
            account: account.pubkey,
            base_account: base_account.pubkey,
        })
    }
}
