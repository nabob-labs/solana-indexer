use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0c")]
pub struct TransferChecked {
    pub amount: u64,
    pub decimals: u8,
}

pub struct TransferCheckedAccounts {
    pub source: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub destination: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for TransferChecked {
    type ArrangedAccounts = TransferCheckedAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [source, mint, destination, authority, remaining_accounts @ ..] = accounts else {
            return None;
        };

        Some(TransferCheckedAccounts {
            source: source.pubkey,
            mint: mint.pubkey,
            destination: destination.pubkey,
            authority: authority.pubkey,
            remaining_accounts: remaining_accounts.to_vec(),
        })
    }
}
