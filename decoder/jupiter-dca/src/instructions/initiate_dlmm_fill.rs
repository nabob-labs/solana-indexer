use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x9bc150795b93febb")]
pub struct InitiateDlmmFill {}

pub struct InitiateDlmmFillInstructionAccounts {
    pub keeper: solana_pubkey::Pubkey,
    pub dca: solana_pubkey::Pubkey,
    pub input_mint: solana_pubkey::Pubkey,
    pub keeper_in_ata: solana_pubkey::Pubkey,
    pub in_ata: solana_pubkey::Pubkey,
    pub out_ata: solana_pubkey::Pubkey,
    pub instructions_sysvar: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitiateDlmmFill {
    type ArrangedAccounts = InitiateDlmmFillInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, dca, input_mint, keeper_in_ata, in_ata, out_ata, instructions_sysvar, system_program, token_program, associated_token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitiateDlmmFillInstructionAccounts {
            keeper: keeper.pubkey,
            dca: dca.pubkey,
            input_mint: input_mint.pubkey,
            keeper_in_ata: keeper_in_ata.pubkey,
            in_ata: in_ata.pubkey,
            out_ata: out_ata.pubkey,
            instructions_sysvar: instructions_sysvar.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
        })
    }
}
