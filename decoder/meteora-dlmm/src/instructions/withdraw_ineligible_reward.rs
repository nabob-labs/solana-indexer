use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x94ce2ac3f7316708")]
pub struct WithdrawIneligibleReward {
    pub reward_index: u64,
}

pub struct WithdrawIneligibleRewardInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub reward_vault: solana_sdk::pubkey::Pubkey,
    pub reward_mint: solana_sdk::pubkey::Pubkey,
    pub funder_token_account: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub bin_array: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for WithdrawIneligibleReward {
    type ArrangedAccounts = WithdrawIneligibleRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, reward_vault, reward_mint, funder_token_account, funder, bin_array, token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawIneligibleRewardInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            reward_vault: reward_vault.pubkey,
            reward_mint: reward_mint.pubkey,
            funder_token_account: funder_token_account.pubkey,
            funder: funder.pubkey,
            bin_array: bin_array.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
