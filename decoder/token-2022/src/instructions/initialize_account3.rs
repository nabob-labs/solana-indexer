use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x12")]
pub struct InitializeAccount3 {
    pub owner: solana_sdk::pubkey::Pubkey,
}

pub struct InitializeAccount3InstructionAccounts {
    pub account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeAccount3 {
    type ArrangedAccounts = InitializeAccount3InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeAccount3InstructionAccounts {
            account: account.pubkey,
            mint: mint.pubkey,
        })
    }
}
