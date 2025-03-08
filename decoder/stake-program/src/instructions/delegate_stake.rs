use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x326e5fb3c24b8cf6")]
pub struct DelegateStake {}

pub struct DelegateStakeInstructionAccounts {
    pub stake: solana_sdk::pubkey::Pubkey,
    pub vote: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub stake_history: solana_sdk::pubkey::Pubkey,
    pub stake_config: solana_sdk::pubkey::Pubkey,
    pub stake_authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for DelegateStake {
    type ArrangedAccounts = DelegateStakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [stake, vote, clock, stake_history, stake_config, stake_authority, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DelegateStakeInstructionAccounts {
            stake: stake.pubkey,
            vote: vote.pubkey,
            clock: clock.pubkey,
            stake_history: stake_history.pubkey,
            stake_config: stake_config.pubkey,
            stake_authority: stake_authority.pubkey,
        })
    }
}
