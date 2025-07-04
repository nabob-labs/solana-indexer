use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x08")]
pub struct Burn {
    pub amount: u64,
}

pub struct BurnInstructionAccounts {
    pub account: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Burn {
    type ArrangedAccounts = BurnInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, mint, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(BurnInstructionAccounts {
            account: account.pubkey,
            mint: mint.pubkey,
            authority: authority.pubkey,
        })
    }
}
