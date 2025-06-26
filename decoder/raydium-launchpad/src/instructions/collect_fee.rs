use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x3cadf767045d8230")]
pub struct CollectFee {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CollectFeeInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub recipient_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CollectFee {
    type ArrangedAccounts = CollectFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, authority, pool_state, global_config, quote_vault, quote_mint, recipient_token_account, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CollectFeeInstructionAccounts {
            owner: owner.pubkey,
            authority: authority.pubkey,
            pool_state: pool_state.pubkey,
            global_config: global_config.pubkey,
            quote_vault: quote_vault.pubkey,
            quote_mint: quote_mint.pubkey,
            recipient_token_account: recipient_token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
