use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x9527de9bd37c981a")]
pub struct SellExactIn {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
    pub share_fee_rate: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SellExactInInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub platform_config: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub user_base_token: solana_pubkey::Pubkey,
    pub user_quote_token: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub base_token_mint: solana_pubkey::Pubkey,
    pub quote_token_mint: solana_pubkey::Pubkey,
    pub base_token_program: solana_pubkey::Pubkey,
    pub quote_token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SellExactIn {
    type ArrangedAccounts = SellExactInInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, authority, global_config, platform_config, pool_state, user_base_token, user_quote_token, base_vault, quote_vault, base_token_mint, quote_token_mint, base_token_program, quote_token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SellExactInInstructionAccounts {
            payer: payer.pubkey,
            authority: authority.pubkey,
            global_config: global_config.pubkey,
            platform_config: platform_config.pubkey,
            pool_state: pool_state.pubkey,
            user_base_token: user_base_token.pubkey,
            user_quote_token: user_quote_token.pubkey,
            base_vault: base_vault.pubkey,
            quote_vault: quote_vault.pubkey,
            base_token_mint: base_token_mint.pubkey,
            quote_token_mint: quote_token_mint.pubkey,
            base_token_program: base_token_program.pubkey,
            quote_token_program: quote_token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
