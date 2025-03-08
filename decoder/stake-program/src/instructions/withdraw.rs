use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xb712469c946da122")]
pub struct Withdraw {
    pub lamports: u64,
}

pub struct WithdrawInstructionAccounts {
    pub from: solana_sdk::pubkey::Pubkey,
    pub to: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub stake_history: solana_sdk::pubkey::Pubkey,
    pub withdraw_authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Withdraw {
    type ArrangedAccounts = WithdrawInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [from, to, clock, stake_history, withdraw_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(WithdrawInstructionAccounts {
            from: from.pubkey,
            to: to.pubkey,
            clock: clock.pubkey,
            stake_history: stake_history.pubkey,
            withdraw_authority: withdraw_authority.pubkey,
        })
    }
}
