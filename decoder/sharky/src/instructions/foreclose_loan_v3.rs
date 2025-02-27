use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x88b8323ab75c3fd8")]
pub struct ForecloseLoanV3 {}

pub struct ForecloseLoanV3InstructionAccounts {
    pub loan: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub escrow_collateral_token_account: solana_sdk::pubkey::Pubkey,
    pub collateral_mint: solana_sdk::pubkey::Pubkey,
    pub borrower: solana_sdk::pubkey::Pubkey,
    pub lender: solana_sdk::pubkey::Pubkey,
    pub lender_collateral_token_account: solana_sdk::pubkey::Pubkey,
    pub borrower_collateral_token_account: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub mpl_token_metadata_program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ForecloseLoanV3 {
    type ArrangedAccounts = ForecloseLoanV3InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [loan, escrow, escrow_collateral_token_account, collateral_mint, borrower, lender, lender_collateral_token_account, borrower_collateral_token_account, metadata, edition, system_program, token_program, associated_token_program, rent, mpl_token_metadata_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ForecloseLoanV3InstructionAccounts {
            loan: loan.pubkey,
            escrow: escrow.pubkey,
            escrow_collateral_token_account: escrow_collateral_token_account.pubkey,
            collateral_mint: collateral_mint.pubkey,
            borrower: borrower.pubkey,
            lender: lender.pubkey,
            lender_collateral_token_account: lender_collateral_token_account.pubkey,
            borrower_collateral_token_account: borrower_collateral_token_account.pubkey,
            metadata: metadata.pubkey,
            edition: edition.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            rent: rent.pubkey,
            mpl_token_metadata_program: mpl_token_metadata_program.pubkey,
        })
    }
}
