use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe455b9704e4f4d02")]
pub struct SetTokenLedger {}

pub struct SetTokenLedgerInstructionAccounts {
    pub token_ledger: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetTokenLedger {
    type ArrangedAccounts = SetTokenLedgerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_ledger, token_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetTokenLedgerInstructionAccounts {
            token_ledger: token_ledger.pubkey,
            token_account: token_account.pubkey,
        })
    }
}
