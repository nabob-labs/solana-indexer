use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x87802f4d0f98f031")]
pub struct OpenPosition {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub tick_array_lower_start_index: i32,
    pub tick_array_upper_start_index: i32,
    pub liquidity: u128,
    pub amount0_max: u64,
    pub amount1_max: u64,
}

#[derive(Debug, PartialEq)]
pub struct OpenPositionInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub position_nft_owner: solana_pubkey::Pubkey,
    pub position_nft_mint: solana_pubkey::Pubkey,
    pub position_nft_account: solana_pubkey::Pubkey,
    pub metadata_account: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub protocol_position: solana_pubkey::Pubkey,
    pub tick_array_lower: solana_pubkey::Pubkey,
    pub tick_array_upper: solana_pubkey::Pubkey,
    pub personal_position: solana_pubkey::Pubkey,
    pub token_account0: solana_pubkey::Pubkey,
    pub token_account1: solana_pubkey::Pubkey,
    pub token_vault0: solana_pubkey::Pubkey,
    pub token_vault1: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub metadata_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for OpenPosition {
    type ArrangedAccounts = OpenPositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, position_nft_owner, position_nft_mint, position_nft_account, metadata_account, pool_state, protocol_position, tick_array_lower, tick_array_upper, personal_position, token_account0, token_account1, token_vault0, token_vault1, rent, system_program, token_program, associated_token_program, metadata_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(OpenPositionInstructionAccounts {
            payer: payer.pubkey,
            position_nft_owner: position_nft_owner.pubkey,
            position_nft_mint: position_nft_mint.pubkey,
            position_nft_account: position_nft_account.pubkey,
            metadata_account: metadata_account.pubkey,
            pool_state: pool_state.pubkey,
            protocol_position: protocol_position.pubkey,
            tick_array_lower: tick_array_lower.pubkey,
            tick_array_upper: tick_array_upper.pubkey,
            personal_position: personal_position.pubkey,
            token_account0: token_account0.pubkey,
            token_account1: token_account1.pubkey,
            token_vault0: token_vault0.pubkey,
            token_vault1: token_vault1.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            metadata_program: metadata_program.pubkey,
        })
    }
}
