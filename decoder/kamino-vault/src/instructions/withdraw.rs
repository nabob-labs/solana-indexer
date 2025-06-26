use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xb712469c946da122")]
pub struct Withdraw {
    pub shares_amount: u64,
}

pub struct WithdrawInstructionAccounts {
    pub withdraw_from_available: solana_pubkey::Pubkey,
    pub withdraw_from_reserve_accounts: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Withdraw {
    type ArrangedAccounts = WithdrawInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [withdraw_from_available, withdraw_from_reserve_accounts, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(WithdrawInstructionAccounts {
            withdraw_from_available: withdraw_from_available.pubkey,
            withdraw_from_reserve_accounts: withdraw_from_reserve_accounts.pubkey,
        })
    }
}
