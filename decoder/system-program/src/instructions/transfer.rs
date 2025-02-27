use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x02000000")]
pub struct Transfer {
    pub lamports: u64,
}

pub struct TransferAccounts {
    pub from: solana_sdk::pubkey::Pubkey,
    pub to: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Transfer {
    type ArrangedAccounts = TransferAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [funding_account, recipient_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(TransferAccounts {
            from: funding_account.pubkey,
            to: recipient_account.pubkey,
        })
    }
}
