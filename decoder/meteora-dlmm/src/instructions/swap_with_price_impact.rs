use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x38ade6d0ade49ccd")]
pub struct SwapWithPriceImpact {
    pub amount_in: u64,
    pub active_id: Option<i32>,
    pub max_price_impact_bps: u16,
}

pub struct SwapWithPriceImpactInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_sdk::pubkey::Pubkey,
    pub reserve_x: solana_sdk::pubkey::Pubkey,
    pub reserve_y: solana_sdk::pubkey::Pubkey,
    pub user_token_in: solana_sdk::pubkey::Pubkey,
    pub user_token_out: solana_sdk::pubkey::Pubkey,
    pub token_x_mint: solana_sdk::pubkey::Pubkey,
    pub token_y_mint: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub host_fee_in: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub token_x_program: solana_sdk::pubkey::Pubkey,
    pub token_y_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SwapWithPriceImpact {
    type ArrangedAccounts = SwapWithPriceImpactInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, bin_array_bitmap_extension, reserve_x, reserve_y, user_token_in, user_token_out, token_x_mint, token_y_mint, oracle, host_fee_in, user, token_x_program, token_y_program, event_authority, program, remaining_accounts @ ..] =
            accounts
        else {
            return None;
        };

        Some(SwapWithPriceImpactInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            bin_array_bitmap_extension: bin_array_bitmap_extension.pubkey,
            reserve_x: reserve_x.pubkey,
            reserve_y: reserve_y.pubkey,
            user_token_in: user_token_in.pubkey,
            user_token_out: user_token_out.pubkey,
            token_x_mint: token_x_mint.pubkey,
            token_y_mint: token_y_mint.pubkey,
            oracle: oracle.pubkey,
            host_fee_in: host_fee_in.pubkey,
            user: user.pubkey,
            token_x_program: token_x_program.pubkey,
            token_y_program: token_y_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            remaining_accounts: remaining_accounts.to_vec(),
        })
    }
}
