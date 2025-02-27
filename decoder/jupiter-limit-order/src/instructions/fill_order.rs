use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe87a7319c78f88a2")]
pub struct FillOrder {
    pub making_amount: u64,
    pub max_taking_amount: u64,
}

pub struct FillOrderInstructionAccounts {
    pub order: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub maker: solana_sdk::pubkey::Pubkey,
    pub taker: solana_sdk::pubkey::Pubkey,
    pub taker_output_account: solana_sdk::pubkey::Pubkey,
    pub maker_output_account: solana_sdk::pubkey::Pubkey,
    pub taker_input_account: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub program_fee_account: solana_sdk::pubkey::Pubkey,
    pub referral: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for FillOrder {
    type ArrangedAccounts = FillOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [order, reserve, maker, taker, taker_output_account, maker_output_account, taker_input_account, fee_authority, program_fee_account, referral, token_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(FillOrderInstructionAccounts {
            order: order.pubkey,
            reserve: reserve.pubkey,
            maker: maker.pubkey,
            taker: taker.pubkey,
            taker_output_account: taker_output_account.pubkey,
            maker_output_account: maker_output_account.pubkey,
            taker_input_account: taker_input_account.pubkey,
            fee_authority: fee_authority.pubkey,
            program_fee_account: program_fee_account.pubkey,
            referral: referral.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
