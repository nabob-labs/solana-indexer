use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x02")]
pub struct DepositAllTokenTypes {
    pub pool_token_amount: u64,
    pub maximum_token_a_amount: u64,
    pub maximum_token_b_amount: u64,
}

pub struct DepositAllTokenTypesInstructionAccounts {
    pub swap: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub user_transfer_authority: solana_pubkey::Pubkey,
    pub deposit_token_a: solana_pubkey::Pubkey,
    pub deposit_token_b: solana_pubkey::Pubkey,
    pub swap_token_a: solana_pubkey::Pubkey,
    pub swap_token_b: solana_pubkey::Pubkey,
    pub pool_mint: solana_pubkey::Pubkey,
    pub destination: solana_pubkey::Pubkey,
    pub token_a_mint: solana_pubkey::Pubkey,
    pub token_b_mint: solana_pubkey::Pubkey,
    pub token_a_program: solana_pubkey::Pubkey,
    pub token_b_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for DepositAllTokenTypes {
    type ArrangedAccounts = DepositAllTokenTypesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [swap, authority, user_transfer_authority, deposit_token_a, deposit_token_b, swap_token_a, swap_token_b, pool_mint, destination, token_a_mint, token_b_mint, token_a_program, token_b_program, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositAllTokenTypesInstructionAccounts {
            swap: swap.pubkey,
            authority: authority.pubkey,
            user_transfer_authority: user_transfer_authority.pubkey,
            deposit_token_a: deposit_token_a.pubkey,
            deposit_token_b: deposit_token_b.pubkey,
            swap_token_a: swap_token_a.pubkey,
            swap_token_b: swap_token_b.pubkey,
            pool_mint: pool_mint.pubkey,
            destination: destination.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            token_a_program: token_a_program.pubkey,
            token_b_program: token_b_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
