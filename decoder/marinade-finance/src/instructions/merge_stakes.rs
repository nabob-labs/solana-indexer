use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xd8248de1f34e7ded")]
pub struct MergeStakes {
    pub destination_stake_index: u32,
    pub source_stake_index: u32,
    pub validator_index: u32,
}

pub struct MergeStakesInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub stake_list: solana_pubkey::Pubkey,
    pub validator_list: solana_pubkey::Pubkey,
    pub destination_stake: solana_pubkey::Pubkey,
    pub source_stake: solana_pubkey::Pubkey,
    pub stake_deposit_authority: solana_pubkey::Pubkey,
    pub stake_withdraw_authority: solana_pubkey::Pubkey,
    pub operational_sol_account: solana_pubkey::Pubkey,
    pub clock: solana_pubkey::Pubkey,
    pub stake_history: solana_pubkey::Pubkey,
    pub stake_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for MergeStakes {
    type ArrangedAccounts = MergeStakesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, stake_list, validator_list, destination_stake, source_stake, stake_deposit_authority, stake_withdraw_authority, operational_sol_account, clock, stake_history, stake_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(MergeStakesInstructionAccounts {
            state: state.pubkey,
            stake_list: stake_list.pubkey,
            validator_list: validator_list.pubkey,
            destination_stake: destination_stake.pubkey,
            source_stake: source_stake.pubkey,
            stake_deposit_authority: stake_deposit_authority.pubkey,
            stake_withdraw_authority: stake_withdraw_authority.pubkey,
            operational_sol_account: operational_sol_account.pubkey,
            clock: clock.pubkey,
            stake_history: stake_history.pubkey,
            stake_program: stake_program.pubkey,
        })
    }
}
