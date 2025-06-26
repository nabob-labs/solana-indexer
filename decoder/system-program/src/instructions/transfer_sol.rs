use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x02000000")]
pub struct TransferSol {
    pub amount: u64,
}

pub struct TransferSolInstructionAccounts {
    pub source: solana_pubkey::Pubkey,
    pub destination: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for TransferSol {
    type ArrangedAccounts = TransferSolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [source, destination, _remaining @ ..] = accounts else {
            return None;
        };

        Some(TransferSolInstructionAccounts {
            source: source.pubkey,
            destination: destination.pubkey,
        })
    }
}
