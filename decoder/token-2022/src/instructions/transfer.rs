use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x03")]
pub struct Transfer {
    pub amount: u64,
}

pub struct TransferInstructionAccounts {
    pub source: solana_pubkey::Pubkey,
    pub destination: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Transfer {
    type ArrangedAccounts = TransferInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [source, destination, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(TransferInstructionAccounts {
            source: source.pubkey,
            destination: destination.pubkey,
            authority: authority.pubkey,
        })
    }
}
