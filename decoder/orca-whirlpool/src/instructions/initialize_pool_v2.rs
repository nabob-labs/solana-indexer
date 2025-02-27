use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xcf2d57f21b3fcc43")]
pub struct InitializePoolV2 {
    pub tick_spacing: u16,
    pub initial_sqrt_price: u128,
}

pub struct InitializePoolV2InstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub token_mint_a: solana_sdk::pubkey::Pubkey,
    pub token_mint_b: solana_sdk::pubkey::Pubkey,
    pub token_badge_a: solana_sdk::pubkey::Pubkey,
    pub token_badge_b: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub token_vault_a: solana_sdk::pubkey::Pubkey,
    pub token_vault_b: solana_sdk::pubkey::Pubkey,
    pub fee_tier: solana_sdk::pubkey::Pubkey,
    pub token_program_a: solana_sdk::pubkey::Pubkey,
    pub token_program_b: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializePoolV2 {
    type ArrangedAccounts = InitializePoolV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whirlpools_config, token_mint_a, token_mint_b, token_badge_a, token_badge_b, funder, whirlpool, token_vault_a, token_vault_b, fee_tier, token_program_a, token_program_b, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializePoolV2InstructionAccounts {
            whirlpools_config: whirlpools_config.pubkey,
            token_mint_a: token_mint_a.pubkey,
            token_mint_b: token_mint_b.pubkey,
            token_badge_a: token_badge_a.pubkey,
            token_badge_b: token_badge_b.pubkey,
            funder: funder.pubkey,
            whirlpool: whirlpool.pubkey,
            token_vault_a: token_vault_a.pubkey,
            token_vault_b: token_vault_b.pubkey,
            fee_tier: fee_tier.pubkey,
            token_program_a: token_program_a.pubkey,
            token_program_b: token_program_b.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
