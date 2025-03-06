use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x7340e24e21d369a2")]
pub struct FulfillFlashFill {
    pub repay_amount: u64,
}

pub struct FulfillFlashFillInstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub dca: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub output_mint: solana_sdk::pubkey::Pubkey,
    pub keeper_in_ata: solana_sdk::pubkey::Pubkey,
    pub in_ata: solana_sdk::pubkey::Pubkey,
    pub out_ata: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub fee_ata: solana_sdk::pubkey::Pubkey,
    pub instructions_sysvar: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for FulfillFlashFill {
    type ArrangedAccounts = FulfillFlashFillInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let keeper = accounts.get(0)?;
        let dca = accounts.get(1)?;
        let input_mint = accounts.get(2)?;
        let output_mint = accounts.get(3)?;
        let keeper_in_ata = accounts.get(4)?;
        let in_ata = accounts.get(5)?;
        let out_ata = accounts.get(6)?;
        let fee_authority = accounts.get(7)?;
        let fee_ata = accounts.get(8)?;
        let instructions_sysvar = accounts.get(9)?;
        let system_program = accounts.get(10)?;
        let token_program = accounts.get(11)?;
        let associated_token_program = accounts.get(12)?;
        let event_authority = accounts.get(13)?;
        let program = accounts.get(14)?;

        Some(FulfillFlashFillInstructionAccounts {
            keeper: keeper.pubkey,
            dca: dca.pubkey,
            input_mint: input_mint.pubkey,
            output_mint: output_mint.pubkey,
            keeper_in_ata: keeper_in_ata.pubkey,
            in_ata: in_ata.pubkey,
            out_ata: out_ata.pubkey,
            fee_authority: fee_authority.pubkey,
            fee_ata: fee_ata.pubkey,
            instructions_sysvar: instructions_sysvar.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
