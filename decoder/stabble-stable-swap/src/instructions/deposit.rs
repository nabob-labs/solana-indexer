use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xf223c68952e1f2b6")]
pub struct Deposit {
    pub amounts: Vec<u64>,
    pub minimum_amount_out: u64,
}

pub struct DepositInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub user_pool_token: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub pool_authority: solana_pubkey::Pubkey,
    pub vault: solana_pubkey::Pubkey,
    pub vault_authority: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_program_2022: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Deposit {
    type ArrangedAccounts = DepositInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, user_pool_token, mint, pool, pool_authority, vault, vault_authority, token_program, token_program_2022, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositInstructionAccounts {
            user: user.pubkey,
            user_pool_token: user_pool_token.pubkey,
            mint: mint.pubkey,
            pool: pool.pubkey,
            pool_authority: pool_authority.pubkey,
            vault: vault.pubkey,
            vault_authority: vault_authority.pubkey,
            token_program: token_program.pubkey,
            token_program_2022: token_program_2022.pubkey,
        })
    }
}
