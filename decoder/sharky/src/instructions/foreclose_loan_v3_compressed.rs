use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xc2c105c17385e7c5")]
pub struct ForecloseLoanV3Compressed {
    pub cnft_root: [u8; 32],
    pub cnft_data_hash: [u8; 32],
    pub cnft_creator_hash: [u8; 32],
    pub cnft_nonce: u64,
    pub cnft_index: u32,
}

pub struct ForecloseLoanV3CompressedInstructionAccounts {
    pub loan: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub borrower: solana_sdk::pubkey::Pubkey,
    pub lender: solana_sdk::pubkey::Pubkey,
    pub tree_authority: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
    pub merkle_tree: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub mpl_bubblegum_program: solana_sdk::pubkey::Pubkey,
    pub compression_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ForecloseLoanV3Compressed {
    type ArrangedAccounts = ForecloseLoanV3CompressedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [loan, escrow, borrower, lender, tree_authority, log_wrapper, merkle_tree, system_program, token_program, mpl_bubblegum_program, compression_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ForecloseLoanV3CompressedInstructionAccounts {
            loan: loan.pubkey,
            escrow: escrow.pubkey,
            borrower: borrower.pubkey,
            lender: lender.pubkey,
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
