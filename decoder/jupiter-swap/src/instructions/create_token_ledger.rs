use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xe8f2c5fdf08f8134")]
pub struct CreateTokenLedger {}

pub struct CreateTokenLedgerInstructionAccounts {
    pub token_ledger: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CreateTokenLedger {
    type ArrangedAccounts = CreateTokenLedgerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_ledger, payer, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CreateTokenLedgerInstructionAccounts {
            token_ledger: token_ledger.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
