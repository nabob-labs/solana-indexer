use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x1f")]
pub struct CreateNativeMint {}

pub struct CreateNativeMintInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub native_mint: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CreateNativeMint {
    type ArrangedAccounts = CreateNativeMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, native_mint, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CreateNativeMintInstructionAccounts {
            payer: payer.pubkey,
            native_mint: native_mint.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
