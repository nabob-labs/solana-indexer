use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0ab1a1eefa257818")]
pub struct Unrestake {
    pub amount: u64,
}

pub struct UnrestakeInstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub lst_mint: solana_pubkey::Pubkey,
    pub lst_ata: solana_pubkey::Pubkey,
    pub rst_ata: solana_pubkey::Pubkey,
    pub rst_mint: solana_pubkey::Pubkey,
    pub vault: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Unrestake {
    type ArrangedAccounts = UnrestakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [signer, lst_mint, lst_ata, rst_ata, rst_mint, vault, pool, associated_token_program, token_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(UnrestakeInstructionAccounts {
            signer: signer.pubkey,
            lst_mint: lst_mint.pubkey,
            lst_ata: lst_ata.pubkey,
            rst_ata: rst_ata.pubkey,
            rst_mint: rst_mint.pubkey,
            vault: vault.pubkey,
            pool: pool.pubkey,
            associated_token_program: associated_token_program.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
