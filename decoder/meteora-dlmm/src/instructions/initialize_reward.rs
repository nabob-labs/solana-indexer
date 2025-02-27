use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x5f87c0c4f281e644")]
pub struct InitializeReward {
    pub reward_index: u64,
    pub reward_duration: u64,
    pub funder: solana_sdk::pubkey::Pubkey,
}

pub struct InitializeRewardInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub reward_vault: solana_sdk::pubkey::Pubkey,
    pub reward_mint: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeReward {
    type ArrangedAccounts = InitializeRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, reward_vault, reward_mint, admin, token_program, system_program, rent, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeRewardInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            reward_vault: reward_vault.pubkey,
            reward_mint: reward_mint.pubkey,
            admin: admin.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
