use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x9f9ff5a8bf9a6406")]
pub struct RepayLoanV3Compressed {
    pub cnft_root: [u8; 32],
    pub cnft_data_hash: [u8; 32],
    pub cnft_creator_hash: [u8; 32],
    pub cnft_nonce: u64,
    pub cnft_index: u32,
}

pub struct RepayLoanV3CompressedInstructionAccounts {
    pub loan: solana_pubkey::Pubkey,
    pub borrower: solana_pubkey::Pubkey,
    pub lender: solana_pubkey::Pubkey,
    pub escrow: solana_pubkey::Pubkey,
    pub value_mint: solana_pubkey::Pubkey,
    pub order_book: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
    pub tree_authority: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
    pub merkle_tree: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub mpl_bubblegum_program: solana_pubkey::Pubkey,
    pub compression_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for RepayLoanV3Compressed {
    type ArrangedAccounts = RepayLoanV3CompressedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [loan, borrower, lender, escrow, value_mint, order_book, fee_authority, tree_authority, log_wrapper, merkle_tree, system_program, token_program, mpl_bubblegum_program, compression_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RepayLoanV3CompressedInstructionAccounts {
            loan: loan.pubkey,
            borrower: borrower.pubkey,
            lender: lender.pubkey,
            escrow: escrow.pubkey,
            value_mint: value_mint.pubkey,
            order_book: order_book.pubkey,
            fee_authority: fee_authority.pubkey,
            tree_authority: tree_authority.pubkey,
            log_wrapper: log_wrapper.pubkey,
            merkle_tree: merkle_tree.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            mpl_bubblegum_program: mpl_bubblegum_program.pubkey,
            compression_program: compression_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
