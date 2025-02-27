use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xf02f99440dbee12a")]
pub struct PreFlashFillOrder {
    pub making_amount: u64,
}

pub struct PreFlashFillOrderInstructionAccounts {
    pub order: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub taker: solana_sdk::pubkey::Pubkey,
    pub taker_output_account: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub input_mint_token_program: solana_sdk::pubkey::Pubkey,
    pub instruction: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for PreFlashFillOrder {
    type ArrangedAccounts = PreFlashFillOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [order, reserve, taker, taker_output_account, input_mint, input_mint_token_program, instruction, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(PreFlashFillOrderInstructionAccounts {
            order: order.pubkey,
            reserve: reserve.pubkey,
            taker: taker.pubkey,
            taker_output_account: taker_output_account.pubkey,
            input_mint: input_mint.pubkey,
            input_mint_token_program: input_mint_token_program.pubkey,
            instruction: instruction.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
