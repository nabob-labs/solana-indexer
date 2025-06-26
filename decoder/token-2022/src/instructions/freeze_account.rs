use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0a")]
pub struct FreezeAccount {}

pub struct FreezeAccountInstructionAccounts {
    pub account: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for FreezeAccount {
    type ArrangedAccounts = FreezeAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, mint, owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(FreezeAccountInstructionAccounts {
            account: account.pubkey,
            mint: mint.pubkey,
            owner: owner.pubkey,
        })
    }
}
