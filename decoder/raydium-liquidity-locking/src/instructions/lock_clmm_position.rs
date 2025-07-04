use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xbc25b38352965449")]
pub struct LockClmmPosition {
    pub with_metadata: bool,
}

pub struct LockClmmPositionInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub position_nft_owner: solana_pubkey::Pubkey,
    pub fee_nft_owner: solana_pubkey::Pubkey,
    pub position_nft_account: solana_pubkey::Pubkey,
    pub personal_position: solana_pubkey::Pubkey,
    pub position_nft_mint: solana_pubkey::Pubkey,
    pub locked_nft_account: solana_pubkey::Pubkey,
    pub locked_position: solana_pubkey::Pubkey,
    pub fee_nft_mint: solana_pubkey::Pubkey,
    pub fee_nft_account: solana_pubkey::Pubkey,
    pub metadata_account: solana_pubkey::Pubkey,
    pub metadata_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub fee_nft_token_program: solana_pubkey::Pubkey,
    pub locked_nft_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for LockClmmPosition {
    type ArrangedAccounts = LockClmmPositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, payer, position_nft_owner, fee_nft_owner, position_nft_account, personal_position, position_nft_mint, locked_nft_account, locked_position, fee_nft_mint, fee_nft_account, metadata_account, metadata_program, associated_token_program, rent, fee_nft_token_program, locked_nft_token_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LockClmmPositionInstructionAccounts {
            authority: authority.pubkey,
            payer: payer.pubkey,
            position_nft_owner: position_nft_owner.pubkey,
            fee_nft_owner: fee_nft_owner.pubkey,
            position_nft_account: position_nft_account.pubkey,
            personal_position: personal_position.pubkey,
            position_nft_mint: position_nft_mint.pubkey,
            locked_nft_account: locked_nft_account.pubkey,
            locked_position: locked_position.pubkey,
            fee_nft_mint: fee_nft_mint.pubkey,
            fee_nft_account: fee_nft_account.pubkey,
            metadata_account: metadata_account.pubkey,
            metadata_program: metadata_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            rent: rent.pubkey,
            fee_nft_token_program: fee_nft_token_program.pubkey,
            locked_nft_token_program: locked_nft_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
