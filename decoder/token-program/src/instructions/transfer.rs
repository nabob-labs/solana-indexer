use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x03")]
pub struct Transfer {
    pub amount: u64,
}

pub struct TransferAccounts {
    pub source: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Transfer {
    type ArrangedAccounts = TransferAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let source = accounts.get(0)?;
        let destination = accounts.get(1)?;
        let authority = accounts.get(2)?;

        Some(TransferAccounts {
            source: source.pubkey,
            destination: destination.pubkey,
            authority: authority.pubkey,
            // TODO: Check
            remaining_accounts: accounts.get(3..).unwrap_or_default().to_vec(),
        })
    }
}
